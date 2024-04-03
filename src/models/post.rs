use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostCreateReq {
    pub title: String,
    pub content: Option<String>
}