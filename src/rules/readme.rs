use crate::{
    readme::{count_emojis, count_headings},
    traits::{Rule, RuleID, RuleResult, Vibe},
};

const EMOJI_COUNT_RULE: &str = "readme-emoji-count";
pub struct EmojiCount;
impl Rule for EmojiCount {
    fn run_impl(
        &self,
        repo: &crate::traits::Repo,
    ) -> Result<Box<dyn crate::traits::RuleResult>, crate::traits::RuleID> {
        let readme = crate::readme::get_readme(repo);
        match readme {
            Some(r) => Ok(Box::new(EmojiCountResult {
                count: count_emojis(&r),
            })),
            None => Err(RuleID(EMOJI_COUNT_RULE)),
        }
    }
}
pub struct EmojiCountResult {
    count: usize,
}
impl RuleResult for EmojiCountResult {
    fn name(&self) -> &'static str {
        EMOJI_COUNT_RULE
    }
    fn vibe_msg(&self) -> String {
        String::from(">30")
    }
    fn msg(&self) -> Option<String> {
        Some(
            format!("Emoji count: {}", self.count)
        )

    }

    fn is_vibe(&self) -> Vibe {
        if self.count > 30 { Vibe::Yes } else { Vibe::No }
    }
}

use crate::rules::macros::threshold_rule;
threshold_rule!(
    id: "readme-llm-words",
    module: llmwords,
    value_function: crate::readme::count_llm_words_repo,
    value_type: usize,
    output_format: "LLM specific words in Readme: {}",
    vibe_compare: gt,
    vibe_threshold: 3_usize
    );

threshold_rule!(
    id: "readme-headings",
    module: readme_headings,
    value_function: crate::readme::count_headings_from_repo,
    value_type: usize,
    output_format: "README.md headings count: {}",
    vibe_compare: gt,
    vibe_threshold: 20_usize
    );

threshold_rule!(
    id: "code-insertions-average",
    module: code_insertions,
    value_function: crate::code::average_code_insertions_per_commit,
    value_type: f64,
    output_format: "Average insertions (IQR): {:.2}",
    vibe_compare: gt,
    vibe_threshold: 500.0
    );

threshold_rule!(
    id: "code-deletions-average",
    module: code_deletions,
    value_function: crate::code::average_code_deletions_per_commit,
    value_type: f64,
    output_format: "Average deletions (IQR): {:.2}",
    vibe_compare: gt,
    vibe_threshold: 50.0
    );

threshold_rule!(
    id: "code-lines-per-hour",
    module: code_lines_per_hour,
    value_function: crate::code::average_lines_per_hour,
    value_type: f64,
    output_format: "Average lines per hour: {:.2}",
    vibe_compare: gt,
    vibe_threshold: 50.0
    );
threshold_rule!(
    id: "code-gap-per-commit",
    module: code_gaps_between_commits,
    value_function: crate::code::average_time_between_commits,
    value_type: f64,
    output_format: "Average time between commits: {:.2} hours",
    vibe_compare: lt,
    vibe_threshold: 5.0
    );
