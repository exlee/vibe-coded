use std::{collections::{HashMap, HashSet}, path::PathBuf};
use cached::proc_macro::once;
use git2::TreeWalkMode;
use strsim::jaro_winkler;

use crate::traits::Repo;

pub fn average_code_insertions_per_commit(repo: &Repo) -> Option<f64> {
    get_repowalk_data(repo)?.insertions
}
pub fn average_code_deletions_per_commit(repo: &Repo) -> Option<f64> {
    get_repowalk_data(repo)?.deletions
}
pub fn average_time_between_commits(repo: &Repo) -> Option<f64> {
    get_repowalk_data(repo)?.time_gaps.map(|v| v/3600.0)
}
pub fn average_lines_per_hour(repo: &Repo) -> Option<f64> {
    let averages = get_repowalk_data(repo)?;
    let ins = averages.insertions?;
    let gaps = averages.time_gaps?;
    Some(ins / (gaps / 3600.0))
}

#[derive(Clone)]
pub struct WalkerResult {
    pub insertions: Option<f64>,
    pub deletions: Option<f64>,
    pub time_gaps: Option<f64>,
    pub files: Vec<String>,
    pub msg_similarity: Option<f64>,

}

#[once(sync_writes=true)]
pub fn get_repowalk_data(repo: &Repo) -> Option<WalkerResult> {
    let Ok(mut walker) = repo.revwalk() else {
        return None
    };
    if walker.push_head().is_err() {
        return None
    };

		let mut insertions: Vec<usize> = Vec::with_capacity(500) ;
		let mut deletions: Vec<usize> = Vec::with_capacity(500) ;
		let mut gaps: Vec<usize> = Vec::with_capacity(500);
		let mut next_commit = None;
		let mut messages: Vec<String> = Vec::with_capacity(500);
		let mut files_hs: HashSet<String> = HashSet::new();

    while let Some(Ok(oid)) = walker.next() {
        let Ok(commit) = repo.find_commit(oid) else {
            continue;
        };

        for te in commit.tree().unwrap().iter() {
            if let Some(p) = te.name() {
                for p in p.split("/") {
                    files_hs.insert(p.to_lowercase());
                }
            }
        }
        if let Some(summary) = commit.summary() {
            messages.push(String::from(summary));
        }

        if next_commit.is_none() {
            next_commit = Some(commit);
            continue;
        };
        let nc = next_commit.unwrap();

				// Stats block
				{
				let diff = repo.diff_tree_to_tree(
    				commit.tree().ok().as_ref(),
    				nc.tree().ok().as_ref(),
    				None
				).unwrap();
    				let stats = diff.stats().unwrap();
    				insertions.push(stats.insertions());
    				deletions.push(stats.deletions());
				}

				gaps.push((nc.time().seconds() - commit.time().seconds()) as usize);
				next_commit = Some(commit);

   				
    }
    Some(WalkerResult {
                insertions: mean_iqr(&mut insertions),
                deletions: mean_iqr(&mut deletions),
                time_gaps: mean_iqr(&mut gaps),
                msg_similarity: analyze_msg_similarity(&messages),
                files: files_hs.iter().cloned().collect(),
    })

}

pub fn analyze_msg_similarity(messages: &[String]) -> Option<f64> {
    let mut messages = messages.to_vec();
    messages.sort_unstable();

    if messages.len() < 5 {
        return None;
    }

    let mut scores: Vec<f64> = messages
        .windows(2)
        .map(|w| jaro_winkler(&w[0], &w[1]))
        .collect();

    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let count = scores.len();
    let q1_idx = count / 4;
    let q3_idx = (count * 3) / 4;

    let q1 = scores[q1_idx];
    let q3 = scores[q3_idx];
    let iqr = q3 - q1;
    let mean: f64 = scores.iter().sum::<f64>() / count as f64;

    Some(mean * (1.0-iqr))
}

trait ToF64 {
    fn to_f64(self) -> f64;
}

macro_rules! impl_to_f64 {
    ($($t:ty),*) => {
        $(
            impl ToF64 for $t {
                fn to_f64(self) -> f64 { self as f64 }
            }
        )*
    };
}
impl_to_f64!(usize, f64);

fn mean_iqr<T: ToF64 + Copy + PartialOrd + std::iter::Sum>(data: &mut [T]) -> Option<f64> {
    if data.len() < 4 {
        return None;
    }

    data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let q1 = data[data.len() / 4].to_f64();
    let q3 = data[data.len() * 3 / 4].to_f64();
    let iqr = q3 - q1;

    let lower_fence = q1 - 1.5 * iqr;
    let upper_fence = q3 + 1.5 * iqr;

    let filtered: Vec<_> = data.iter()
        .filter(|&&x| x.to_f64() >= lower_fence && x.to_f64() <= upper_fence)
        .collect();

    if filtered.is_empty() {
        return None;
    }

    Some(filtered.iter().map(|&&x| x).sum::<T>().to_f64() / filtered.len() as f64)
}

const COMMENT_MAP: [(&str, &str); 47] = [
    ("ada", "--"),
    ("bash", "#"),
    ("c", "//"),
    ("clj", ";"),
    ("cpp", "//"),
    ("cs", "//"),
    ("dart", "//"),
    ("dockerfile", "#"),
    ("el", "#"),
    ("elm", "--"),
    ("erl", "%"),
    ("f90", "!"),
    ("go", "//"),
    ("h", "//"),
    ("hpp", "//"),
    ("hs", "--"),
    ("ini", ";"),
    ("java", "//"),
    ("jl", "#"),
    ("js", "//"),
    ("jsx", "//"),
    ("kak", "#"),
    ("kt", "//"),
    ("lisp", ";"),
    ("lua", "--"),
    ("m", "%"),
    ("makefile", "#"),
    ("php", "//"),
    ("pl", "#"),
    ("py", "#"),
    ("r", "#"),
    ("rb", "#"),
    ("rs", "//"),
    ("scala", "//"),
    ("scm", ";"),
    ("sh", "#"),
    ("sql", "--"),
    ("swift", "//"),
    ("tex", "%"),
    ("toml", "#"),
    ("ts", "//"),
    ("tsx", "//"),
    ("vhdl", "--"),
    ("vim", "\""),
    ("yaml", "#"),
    ("yml", "#"),
    ("zsh", "#"),
];

pub fn count_comment_ratio(repo: &Repo) -> Option<(usize, usize)> {
    let ext_map: HashMap<&str, &str> = HashMap::from(COMMENT_MAP);
    let mut line_count = 0;
    let mut comment_count = 0;
    repo.head()
        .ok()?
        .peel_to_tree()
        .ok()?
        .walk(TreeWalkMode::PreOrder, |_, te| {
            let _: Option<()> = (|| {
                let path = PathBuf::from(te.name().unwrap());
                if let Some(ext) = path.extension().and_then(|s| s.to_str())
                    && let Some(comment) = ext_map.get(ext)
                {
                    let blob = te.to_object(repo).ok()?.peel_to_blob().ok()?;
                    let content = String::from_utf8(blob.content().to_vec()).ok()?;
                    for line in content.lines() {
                        line_count += 1;
                        if line.contains(comment) {
                            comment_count += 1;
                        }
                    }
                }
                Some(())
            })();
            0
        })
        .ok()?;
    Some((comment_count, line_count))
}
