use crate::traits::Repo;
use crate::traits::Rule;
use crate::traits::RuleResult;
use crate::traits::Vibe;

pub enum Age {
    Unknown,
    Days(f64),
}
pub struct RepositoryDevTime;

impl Rule for RepositoryDevTime {
    fn run_impl(&self, repo: &Repo) -> Result<Box<dyn RuleResult>, crate::traits::RuleID> {
 
        let age = match seconds_between_boundary(repo)  {
            None => Age::Unknown,
            Some(delta) => {
                Age::Days(delta as f64 / (24.0 * 3600.0) )
            }
        };

        Ok(Box::new(RepositoryDevTimeResult { age }))
    }
}

pub struct RepositoryDevTimeResult {
    age: Age,
}

impl RuleResult for RepositoryDevTimeResult {
    fn name(&self) -> &'static str {
        "repository-dev-time"
    }
    fn msg(&self) -> Option<String> {
        let msg = match self.age {
            Age::Unknown => String::from("Unknown age"),
            Age::Days(v) => format!("Development over {:4.2} days", v),
        };
        Some(msg)
    }
    fn vibe_msg(&self) -> String {
        String::from("< 14 days")
    }


    fn is_vibe(&self) -> crate::traits::Vibe {
        match self.age {
            Age::Unknown => Vibe::Undecided,
            Age::Days(v) if v > 14.0 => Vibe::No,
            Age::Days(_) => Vibe::Yes,
        }

    }
}

pub(crate) fn get_boundary_ts(repo: &Repo) -> (Option<i64>, Option<i64>) {
    let Ok(mut walker) = repo.revwalk() else {
        return (None, None);
    };
    if walker.push_head().is_err() {
        return (None, None);
    };
    let mut newest_ts = None;
    let mut oldest_ts = None;

    while let Some(Ok(oid)) = walker.next() {
        let Ok(commit) = repo.find_commit(oid) else {
            continue;
        };

        if newest_ts.is_none() {
            newest_ts = Some(commit.time().seconds());
        }
        if commit.parent(0).is_err() {
            oldest_ts = Some(commit.time().seconds());
        }
    }
    (newest_ts, oldest_ts)
}
fn seconds_between_boundary(repo: &Repo) -> Option<usize>{
        match get_boundary_ts(repo)  {
            (None, _) | (_, None) => None,
            (Some(newest), Some(oldest)) => {
                Some((newest - oldest) as usize)
            }
        }
}
