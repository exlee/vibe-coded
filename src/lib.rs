use crate::{repo::clone_repository, traits::RuleResult};

use rayon::prelude::*;

mod rules;
mod traits;
mod rule_formatter;
mod staging_dir;
mod readme;
mod code;
pub mod repo;

pub fn run_rules(url: &str) -> Result<(), anyhow::Error> {

		// Needs to be done or all hell breaks loose on par_iter
    let _= clone_repository(url).unwrap();
		let mut results= rules::all().par_iter().map(|rule| rule.run_url(url)).collect::<Vec<_>>();
		results.sort_by_key(|a| a.name());
		results.sort_by_key(|a| a.is_vibe());
		for r in results {
    		r.render();
		}

    Ok(())
}
