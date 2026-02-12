use crate::{traits::Repo};

macro_rules! rule_run_impl {
    ($rule:ident, $id:ident, $repo_fn:expr, $closure:expr) => {

            impl crate::traits::Rule for $rule {
                fn run_impl(
                    &self,
                    repo: &crate::traits::Repo,
                ) -> Result<Box<dyn crate::traits::RuleResult>, crate::traits::RuleID> {
                    let some_value = $repo_fn(repo);
                    match some_value {
                        Some(r) => Ok(Box::new($closure(r))),
                        None => Err($crate::traits::RuleID($id)),
                    }
                }
            }
    }
}
pub(crate) use rule_run_impl;
macro_rules! threshold_rule {
    (
        id: $rule_id:literal,
        module: $module:ident,
        value_function: $some_fn:expr,
        value_type: $value_ty:ty,
        output_format: $format_str:literal,
        vibe_compare: $cmp_fn:ident,
        vibe_threshold: $cmp_v:expr $(,)?
    ) => {
        pub mod $module {
            use crate::{
                traits::{ RuleID }
            };
            pub struct Rule;

            impl crate::traits::Rule for Rule {
                fn run_impl(
                    &self,
                    repo: &crate::traits::Repo,
                ) -> Result<Box<dyn crate::traits::RuleResult>, crate::traits::RuleID> {
                    let some_value = $some_fn(repo);
                    match some_value {
                        Some(r) => Ok(Box::new(RuleResult {
                            value: r,
                        })),
                        None => Err(RuleID($rule_id)),
                    }
                }
            }
            use crate::{
                traits::{Vibe}
            };
            pub struct RuleResult {
                pub value: $value_ty
            }
            impl crate::traits::RuleResult for RuleResult {
                fn name(&self) -> &'static str {
                    $rule_id
                }
                fn msg(&self) -> Option<String> {
                    Some(
                        format!($format_str, self.value)
                    )
                }

                fn vibe_msg(&self) -> String {
                    $crate::rules::macros::threshold_rule!(@vibe $cmp_fn $cmp_v)
                }

                fn is_vibe(&self) -> Vibe {
                    if self.value.$cmp_fn(&$cmp_v) { Vibe::Yes } else { Vibe::No }
                }

            }
        }
    };
    (@vibe gt $cmp_v:literal) => {
        format!("? > {}", $cmp_v)
    };
    (@vibe lt $cmp_v:literal) => {
        format!("? < {}", $cmp_v)
    }
}
pub(crate) use threshold_rule;
