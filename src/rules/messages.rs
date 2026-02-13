use crate::rules::macros::threshold_rule;


threshold_rule!(
id: "message-similarity",
module: message_similarities,
value_function: crate::messages::get_message_similarities,
value_type: f64,
output_format: "Msg similiarity index: {:.2} ",
vibe_compare: gt,
vibe_threshold: 0.75
);
