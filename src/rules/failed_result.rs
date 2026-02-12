    use crate::{rule_formatter::{ResultType, RuleFormatter}, traits::{RuleResult, Vibe}};

    pub struct FailedResult {
        pub(crate) name: &'static str,
    }

    impl RuleResult for FailedResult {
        fn name(&self) -> &'static str {
            self.name
        }

				fn is_vibe(&self) -> Vibe {
    				Vibe::Undecided
				}
        fn render(&self) {
            RuleFormatter {
                rule_name: self.name(),
                result_type: ResultType::Fail,
                ..Default::default()
            }.print();
        }


  //      fn render(&self, printer: &mut dyn crate::traits::RuleFormatter) {
  //          printer.write_result(self.name(), self.score(), "FAILED");
  //      }
    }

    impl From<&'static str> for FailedResult {
        fn from(value: &'static str) -> Self {
            Self {
                name: value
            }
        }
    }

