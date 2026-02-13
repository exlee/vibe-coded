use crate::{traits::Rule};

pub(crate) mod failed_result;
mod repository_dev_time;
mod repository_age;
mod readme;
mod repository;
mod macros;
mod messages;

pub fn all() -> Vec<&'static dyn Rule> {
    let result: Vec<&'static dyn Rule> = inventory::iter::<crate::traits::RuleReg>.into_iter().map(|s| s.0).collect();
    result
}


