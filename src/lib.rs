use crate::{repo::{clone_repository,path_from_url}};
use anyhow::{Context};

use rayon::prelude::*;

mod rules;
mod traits;
mod rule_formatter;
mod staging_dir;
mod readme;
mod messages;
mod code;
pub mod repo;

pub fn run_rules(url: &str) -> Result<(), anyhow::Error> {

		// Needs to be done or all hell breaks loose on par_iter
    let _= clone_repository(url).unwrap();
		let mut results= rules::all().par_iter().map(|rule| rule.run_url(url)).collect::<Vec<_>>();
		results.sort_by_key(|a| a.name());
		//results.sort_by_key(|a| a.is_vibe());
		for r in results {
    		r.render();
		}

    Ok(())
}

pub fn clean_repo_dir(url: &str) -> Result<(), anyhow::Error> {
    let path = path_from_url(url);
    if path.exists() {
        println!("Cleaning: {}", &path.to_string_lossy());
        std::fs::remove_dir_all(&path).context("Failed to clean repo dir")
    } else {
        Ok(())
    }
}
