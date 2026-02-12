use crate::{
    readme::count_emojis,
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

inventory::submit! {
    crate::traits::RuleReg(&EmojiCount)
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
        Some(format!("Emoji count: {}", self.count))
    }

    fn is_vibe(&self) -> Vibe {
        if self.count > 30 { Vibe::Yes } else { Vibe::No }
    }
}

pub mod readme_llm_words {
    use std::collections::HashMap;

    use crate::rules::macros::rule_run_impl;
    const RULE_ID: &str = "readme-llm-words";

    pub struct Rule;
    pub struct RuleResult {
        count: usize,
        frequent: Vec<(String, usize)>,
    }
    impl From<HashMap<String,usize>> for RuleResult {
        fn from(value: HashMap<String,usize>) -> Self {
            let count = value.values().sum();
            let mut entries: Vec<(String,usize)> = value.into_iter().collect();
            entries.sort_by_key(|s| s.1);
            let frequent: Vec<(String, usize)> = entries.into_iter().rev().take(3).collect();
            Self {
                count,
                frequent

            }
        }
    }
    rule_run_impl!(Rule,RULE_ID,crate::readme::count_llm_words_repo );
    impl crate::traits::RuleResult for RuleResult {
        fn msg(&self) -> Option<String> {
            let freq_str = self.frequent.iter().map(|(k,c)| format!("{k} x{c}")).collect::<Vec<_>>().join(", ");
            let count = self.count;
            let pluralize  = match self.count {
                1 => "",
                _ => "s"
            };
            let context = if count > 0 {
                format!(": {freq_str}")
            } else {
                String::new()
            };

            Some(format!("Found {count} LLM word{pluralize} in README{context}"))

        }
        fn name(&self) -> &'static str {
            RULE_ID
        }

        fn is_vibe(&self) -> crate::traits::Vibe {
            if self.count > 2 {
                crate::traits::Vibe::Yes
            } else {
                crate::traits::Vibe::No
            }
        }
        fn vibe_msg(&self) -> String {
            String::from(">2")
        }
    }
}

//threshold_rule!(
//id: "readme-llm-words",
//module: llmwords,
//value_function: crate::readme::count_llm_words_repo,
//value_type: usize,
//output_format: "LLM specific words in Readme: {}",
//vibe_compare: gt,
//vibe_threshold: 3_usize
//);

use crate::rules::macros::threshold_rule;

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
id: "readme-length",
module: readme_length,
value_function: crate::readme::length_in_words,
value_type: usize,
output_format: "README.md word count: {}",
vibe_compare: gt,
vibe_threshold: 1000_usize
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
