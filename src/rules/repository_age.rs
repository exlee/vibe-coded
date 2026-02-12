use crate::{traits::{Repo, Rule, RuleID, RuleResult, Vibe}};

pub struct RepositoryAge;

impl Rule for RepositoryAge {
    fn run_impl(&self, repo: &Repo) -> Result<Box<dyn RuleResult>, RuleID> {
        Ok(Box::new(RepositoryAgeResult { age_in_s: secs_since_first_commit(repo) }))
    }
}

pub struct RepositoryAgeResult {
    age_in_s: u64
}

impl RuleResult for  RepositoryAgeResult {
    fn name(&self) -> &'static str {
        "repository-age"
    }
    fn is_vibe(&self) -> Vibe {
        if self.age_in_s > 24*3600*14 {
            Vibe::No
        } else {
            Vibe::Yes
        }
    }
    fn vibe_msg(&self) -> String {
            String::from("< 14 days")
    }
    fn msg(&self) -> Option<String> {
        Some(format!("Age of repository: {:.2} days",(self.age_in_s as f64 / (24.0*3600.0))))
    }
}

fn secs_since_first_commit(repo: &Repo) -> u64 {
    let (_, beginning) = super::repository_dev_time::get_boundary_ts(repo);
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let beginning = beginning.unwrap_or(now as i64);
    now - beginning as u64 
}
