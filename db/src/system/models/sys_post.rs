use sea_orm::{entity::prelude::DateTime, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub post_id: Option<String>,
    pub post_code: Option<String>,
    pub post_name: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteReq {
    pub post_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EditReq {
    pub post_id: String,
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, FromQueryResult)]
pub struct Resp {
    pub post_id: String,
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    pub status: String,
    pub remark: String,
    pub created_at: DateTime,
}
