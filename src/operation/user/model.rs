use serde::{Deserialize, Serialize};

// Base

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub last_login_time: String,
}

// Request

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisterReq {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginReq {
    pub name: String,
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserChangePasswordReq {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGetDetailReq {
    pub id: i32,
}

// Response

#[derive(Debug, Serialize, Deserialize)]
pub struct UserShortDetail {
    pub id: i32,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRes {
    pub token: String,
}