use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationReq {
    pub page: i64,
    pub size: i64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationRes<T> {
    pub total: i64,
    pub data: Vec<T>,
}