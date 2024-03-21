use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PostCreateReq {
    pub(crate) title: String,
    pub(crate) content: Option<String>
}