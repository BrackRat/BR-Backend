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


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserChangePasswordReq {
    pub(crate) old_password: String,
    pub(crate) new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PostCreateReq {
    pub(crate) title: String,
    pub(crate) content: Option<String>
}