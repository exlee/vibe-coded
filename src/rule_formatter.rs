use colored::*;

use crate::traits::Vibe;

pub enum ResultType {
    Info,
    Fail,
    Hand,
    Vibe,
    Percentage(usize),
}

impl From<Vibe> for ResultType {
    fn from(value: Vibe) -> Self {
        match value {
            Vibe::Yes => ResultType::Vibe,
            Vibe::No => ResultType::Hand,
            Vibe::Undecided => ResultType::Info,
        }
    }
}

pub struct RuleFormatter {
    pub rule_name: &'static str,
    pub msg: Option<String>,
    pub result_type: ResultType,
    pub context_msg: String,
}

impl RuleFormatter {
    fn fmt_result_type(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = match self.result_type {
            ResultType::Info => "NOTE".blue(),
            ResultType::Vibe => "VIBE".bright_red(),
            ResultType::Hand => "HAND".bright_green(),
            ResultType::Percentage(v) => format!("{:-3}%", v.clamp(0, 100)).normal(),
            ResultType::Fail => "FAIL".bright_red(),
        };
        let color = inner.fgcolor;


        let mut final_string = ColoredString::from(format!("[ {} ]", inner));
        final_string.fgcolor = color;

        let s = final_string.to_string();
        f.write_str(&s)?;

        Ok(())
    }

    pub(crate) fn print(&self) {
        println!("{}", self);
    }
}
impl Default for RuleFormatter {
    fn default() -> Self {
        Self {
            rule_name: Default::default(),
            msg: Default::default(),
            result_type: ResultType::Info,
            context_msg: Default::default()
        }
    }
}
impl std::fmt::Display for RuleFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_result_type(f)?;
        f.write_str(" ")?;
        write!(f, "{:25}", self.rule_name)?;
        f.write_str(" ")?;
        match &self.msg {
            Some(m) => write!(f, "| {}", m),
            None => Ok(()),
        }?;
        if !self.context_msg.is_empty() {
            f.write_str(" ")?;
            write!(f, "({})", self.context_msg)
        } else {
            Ok(())
        }
        
    }
}

mod test {
    #[test]
    fn test_percentage() {
        todo!();
    }
}
