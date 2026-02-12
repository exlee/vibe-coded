use std::{collections::HashSet, fmt::Debug};
use cached::proc_macro::once;

use crate::traits::Repo;

pub struct IQRs {
    insertions: f64,
    deletions: f64,
    time_gaps: f64,
}

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
		let mut gaps: Vec<usize> = Vec::with_capacity(500) ;
		let mut next_commit = None;
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

        if next_commit.is_none() {
            next_commit = Some(commit);
            continue;
        };
        let nc = next_commit.unwrap();

				let diff = repo.diff_tree_to_tree(
    				commit.tree().ok().as_ref(),
    				nc.tree().ok().as_ref(),
    				None
				).unwrap();
				let stats = diff.stats().unwrap();
				insertions.push(stats.insertions());
				deletions.push(stats.deletions());
				gaps.push((nc.time().seconds() - commit.time().seconds()) as usize);


				next_commit = Some(commit);

   				
    }
    Some(WalkerResult {
                insertions: mean_iqr(&mut insertions),
                deletions: mean_iqr(&mut deletions),
                time_gaps: mean_iqr(&mut gaps),
                files: files_hs.iter().cloned().collect(),
    })

}
fn mean_iqr(data: &mut [usize]) -> Option<f64> {
    if data.len() < 4 {
        return None;
    }

    data.sort_unstable();

    let q1 = data[data.len() / 4] as f64;
    let q3 = data[data.len() * 3 / 4] as f64;
    let iqr = q3 - q1;

    let lower_fence = q1 - 1.5 * iqr;
    let upper_fence = q3 + 1.5 * iqr;

    let filtered: Vec<_> = data.iter()
        .filter(|&&x| x as f64 >= lower_fence && x as f64 <= upper_fence)
        .collect();

    if filtered.is_empty() {
        return None;
    }

    Some(filtered.iter().map(|&&x| x).sum::<usize>() as f64 / filtered.len() as f64)
}
