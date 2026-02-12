use crate::{rules::{repository_age::RepositoryAge, repository_dev_time::RepositoryDevTime}, traits::Rule};

pub(crate) mod failed_result;
mod repository_dev_time;
mod repository_age;
mod readme;
mod repository;
mod macros;

pub fn all() -> Vec<Box <dyn Rule>> {
    vec![
        Box::new(RepositoryDevTime{}),
        Box::new(RepositoryAge{}),
        Box::new(readme::EmojiCount{}),
        Box::new(readme::llmwords::Rule{}),
        Box::new(readme::readme_headings::Rule{}),
        Box::new(readme::code_insertions::Rule{}),
        Box::new(readme::code_deletions::Rule{}),
        Box::new(readme::code_lines_per_hour::Rule{}),
        Box::new(readme::code_gaps_between_commits::Rule{}),
        Box::new(repository::llm_files::Rule{}),
    ]
}


