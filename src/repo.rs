use std::collections::HashSet;

use anyhow::Context;
use git2::Repository;

use crate::{staging_dir::StagingDir, traits::Repo};

pub fn clone_repository(url: &str) -> Result<Repository,anyhow::Error> {
    let slug = slug_from_url(url);
    let tmpdir_root = std::env::temp_dir();
    let dest_path = &tmpdir_root.join("vibe-coded").join(slug);
    if dest_path.exists() {
        return git2::Repository::open(dest_path).context("Can't existing open repository");
    };
    let mut destination = StagingDir::try_new(dest_path)?;

    let repo = git2::Repository::clone(url, &destination)?;
    destination.persist();

    Ok(repo)
  
}

fn slug_from_url(url: &str) -> String {
    let sanitized: String = url
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .take(30)
        .collect();
    
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(url, &mut hasher);
    use std::hash::Hasher;
    format!("{}-{:x}", sanitized.trim_matches('-'), hasher.finish())
}

pub fn llm_specific_files(repo: &Repo) -> Option<Vec<String>> {
    crate::code::get_repowalk_data(repo).map(|d| d.files)
}
