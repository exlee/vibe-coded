pub mod llm_files {
    use crate::rules::macros::rule_run_impl;
    const RULE_ID: &str = "repository-llm-files";
    pub const LLM_KEYWORDS: &[&str] = &[
        "aider", "chatgpt", "claude", "clinerules", "continue", "copilot", "cursor",
        "cursorrules", "fabric", "gpt", "llm", "openai", "prompt", "system_prompt",
        "windsurfrules",
    ];
    pub struct Rule;

    rule_run_impl!(Rule, RULE_ID, crate::repo::llm_specific_files);
    pub struct RuleResult {
        pub llm_files: Vec<String>,
    }
    impl From<Vec<String>> for RuleResult {
        fn from(value: Vec<String>) -> Self {
            let mut llm_files = Vec::new();
            for file in &value {
            for key in LLM_KEYWORDS {
                if file.contains(key) {
                    llm_files.push(file.clone());
                }
            }
            };
            Self {
                llm_files
            }
        }
    }

    impl crate::traits::RuleResult for RuleResult {
        fn name(&self) -> &'static str {
            RULE_ID
        }
        fn msg(&self) -> Option<String> {
            if self.llm_files.is_empty() {
                return Some(String::from("no LLM-tool files"));
            }
						let llm_files = self.llm_files.clone();
            let mut result = format!("LLM-tool files found: {}", llm_files.join(", "));
            if result.len() > 60 {
                result.truncate(60);
                result.push_str("...");
            }
            Some(result)
        }

        fn is_vibe(&self) -> crate::traits::Vibe {
            if self.llm_files.is_empty() {
                crate::traits::Vibe::No
            } else {
                crate::traits::Vibe::Yes
            }
        }
    }
}

pub mod comment_lines {
    use crate::rules::macros::rule_run_impl;
    const RULE_ID: &str = "code-comment-ratio";

    rule_run_impl!(Rule, RULE_ID, crate::code::count_comment_ratio);
    struct Rule;
    struct RuleResult {
        comments: usize,
        lines: usize,
        ratio: f64,
    }
    impl From<(usize, usize)> for RuleResult {
        fn from(value: (usize, usize)) -> Self {
            Self {
                comments: value.0,
                lines: value.1,
                ratio: value.0 as f64 /value.1 as f64,
            }
        }
    }
    impl crate::traits::RuleResult for RuleResult {
        fn name(&self) -> &'static str {
            RULE_ID
        }

        fn is_vibe(&self) -> crate::traits::Vibe {
            if self.ratio > 0.05 {
                crate::traits::Vibe::Yes
            } else {
                crate::traits::Vibe::No
            }
        }
        fn vibe_msg(&self) -> String {
            "ratio > 0.05".into()
        }

        fn msg(&self) -> Option<String> {
            let lines = self.lines;
            let comments = self.comments;
            let ratio = self.ratio;
            Some(format!("Lines: {lines}, Comment Lines: {comments}, Ratio: {ratio:.2}"))
        }
    }

}

