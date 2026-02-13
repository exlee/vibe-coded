use crate::{code::get_repowalk_data, traits::Repo};

pub fn get_message_similarities(repo: &Repo) -> Option<f64> {
    let data = get_repowalk_data(repo)?;
    data.msg_similarity
}
