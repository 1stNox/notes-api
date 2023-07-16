use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Note {
    pub _id: String,
    pub index: u64,
    pub title: String,
    pub text: String,
}
