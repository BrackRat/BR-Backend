use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserRegisterReq {
    pub(crate) name: String,
    pub(crate) password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserLoginReq {
    pub(crate) name: String,
    pub(crate) password: String,
}