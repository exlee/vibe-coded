use std::fmt::Write;
use anyhow::anyhow;
use crate::repo::clone_repository;
use crate::rules::failed_result::FailedResult;

use crate::rule_formatter::RuleFormatter;

pub type Repo = git2::Repository;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Vibe {
    Yes,
    No,
    Undecided,
}

pub struct RuleID(pub &'static str);

pub trait Rule: Send + Sync {
    fn run_impl(&self, repo: &Repo) -> Result<Box<dyn RuleResult>, RuleID>;
    fn run_url(&self, url: &str) -> Box<dyn RuleResult> {
        let repo = clone_repository(url).unwrap();
        self.run(&repo)
    }
   fn run(&self, repo: &Repo) -> Box<dyn RuleResult> {

        match self.run_impl(&repo) {
            Ok(r) => r,
            Err(e) => Box::new(FailedResult::from(e.0)),
        }
    }

}

pub trait RuleResult: Send + Sync {
    fn name(&self) -> &'static str;
    fn is_vibe(&self) -> Vibe;
    fn vibe_msg(&self) -> String {
        String::from("")
    }
    fn render(&self) {
        RuleFormatter {
            rule_name: self.name(),
            msg: self.msg(),
            //context_msg: if self.is_vibe() == Vibe::Yes { self.vibe_msg() } else { String::from("") },
            context_msg: self.vibe_msg(),
            result_type: self.is_vibe().into(),
        }.print();
    }
    fn msg(&self) -> Option<String> {
        None
    }

}


