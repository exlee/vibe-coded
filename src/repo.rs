use std::{path::PathBuf, sync::Once};

use anyhow::Context;
use git2::{FetchOptions, Repository};

use crate::{staging_dir::StagingDir, traits::Repo};

static REPO_MSG: Once = Once::new();

pub fn path_from_url(url: &str) -> PathBuf {
    let slug = slug_from_url(url);
    let tmpdir_root = std::env::temp_dir();
    let dest_path = &tmpdir_root.join("vibe-coded").join(slug);
    dest_path.clone()
}
pub fn clone_repository(url: &str) -> Result<Repository, anyhow::Error> {
    let dest_path = &path_from_url(url);
    if dest_path.exists() {
        REPO_MSG.call_once(|| {
            println!("Repository exists at: {}", &dest_path.to_string_lossy());
        });
        return git2::Repository::open(dest_path).context("Can't existing open repository");
    };
    let mut destination = StagingDir::try_new(dest_path)?;
    let mut fo = FetchOptions::new();
    fo.depth(100);
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

	 	println!("Fetching repository, it might take a while");
    let repo = builder.clone(url, &destination.path)?;
    destination.persist();
    REPO_MSG.call_once(|| {
        println!("Repository created at: {}", &dest_path.to_string_lossy());
    });

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
