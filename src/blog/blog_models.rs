use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use prkorm::*;

#[derive(Table, Serialize, Deserialize, FromRow)]
#[table_name("blogs")]
pub struct Blog {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub reads_count: i32,
}

#[derive(Deserialize, Serialize)]
pub struct CreateBlog {
    pub user_id: i32,
    pub title: String,
    pub body: String,
}
