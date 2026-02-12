pub mod llm_files {
    use crate::rules::macros::rule_run_impl;
    const RULE_ID: &str = "repository-llm-files";
    pub const LLM_KEYWORDS: &[&str] = &[
        "aider", "chatgpt", "claude", "clinerules", "continue", "copilot", "cursor",
        "cursorrules", "fabric", "gpt", "llm", "openai", "prompt", "system_prompt",
        "windsurfrules",
    ];
    pub struct Rule;

    rule_run_impl!(Rule, RULE_ID, crate::repo::llm_specific_files, |v| RuleResult { files: v, llm_files: None }.check_files() );
    pub struct RuleResult {
        pub files: Vec<String>,
        pub llm_files: Option<Vec<String>>,

    }
    impl RuleResult {
        fn check_files(self) -> Self {
            let mut llm_files = Vec::new();
            for file in &self.files {
            for key in LLM_KEYWORDS {
                if file.contains(key) {
                    llm_files.push(file.clone());
                }
            }
            };
            Self {
                files: self.files,
                llm_files: if llm_files.is_empty() { None } else { Some(llm_files) },
            }
        }
    }
    impl crate::traits::RuleResult for RuleResult {
        fn name(&self) -> &'static str {
            RULE_ID
        }
        fn msg(&self) -> Option<String> {
            if self.llm_files.is_none() {
                return Some(String::from("no LLM-tool files"));
            }
						let llm_files = self.llm_files.clone();
            let mut result = format!("LLM-tool files found: {}", llm_files.unwrap().join(", "));
            if result.len() > 60 {
                result.truncate(60);
                result.push_str("...");
            }
            Some(result)
        }

        fn is_vibe(&self) -> crate::traits::Vibe {
            if self.llm_files.is_some() {
                crate::traits::Vibe::Yes
            } else {
                crate::traits::Vibe::No
            }
        }
    }
}
