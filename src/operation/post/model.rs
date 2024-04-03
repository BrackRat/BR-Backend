use serde::{Deserialize, Serialize};
use crate::operation::user;

// Base
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub user: user::User,
}

// Request
#[derive(Debug, Serialize, Deserialize)]
pub struct PostCreateReq {
    pub title: String,
    pub content: Option<String>,
}

// Response
#[derive(Debug, Serialize, Deserialize)]
pub struct PostShortRes {
    pub cuid: String,
    pub title: String,
    pub author: user::UserShortDetail
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostDetailRes {
    pub title: String,
    pub content: Option<String>,
    pub author: user::UserShortDetail
}