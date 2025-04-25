use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::{Serialize, Deserialize};
use sqlx::{prelude::FromRow, MySqlPool};
use axum::extract::{Path, Json};

/*
assumptions:
query: parsing an object into sql
query_as: reading from
query!: side effect of another function
*/

#[derive(Serialize, Deserialize, FromRow)]
pub struct Blog {
    id: i32,
    title: String,
    body: String,
    reads: i32,
}

#[derive(Deserialize, Serialize)]
pub struct CreateBlog {
    title: String,
    body: String,
    reads: i32
}

impl Blog {
    pub async fn create_a_blog(State(db): State<MySqlPool>, Json(body): Json<CreateBlog>) -> impl IntoResponse { // bit of sql here since we are writing to the database
        let res = sqlx::query("
            INSERT INTO blogs
            (title, body, `reads`) values(?, ?, ?)
        ").bind(body.title.clone())
        .bind(body.body.clone())
        .bind(body.reads)
        .execute(&db)
        .await;

        match res {
            Ok(blog) => (StatusCode::CREATED, Json(Self{
                id: blog.last_insert_id() as i32,
                body: body.body.clone(),
                reads: 0,
                title: body.title.clone()
            })).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }

    pub async fn fetch_all_blogs(State(db): State<MySqlPool>) -> impl IntoResponse {
        let res: Result<Vec<Self>, sqlx::Error> = sqlx::query_as(
            "
            SELECT id, title, `reads`, body, created_at
            from blogs
            "
        ).fetch_all(&db)
        .await;
        
        match res {
            Ok(blogs) => (StatusCode::OK, Json(blogs)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }

    pub async fn fetch_single_blog(State(db): State<MySqlPool>, Path(id): Path<u32>) -> impl IntoResponse {
        sqlx::query( // side effect 
            "
            UPDATE blogs 
            SET `reads` =  `reads` + 1
            WHERE id = ?
            "
        ).bind(id)
        .execute(&db)
        .await.unwrap();


        let res = sqlx::query_as::<_, Self>("
            SELECT id, title, body, `reads`
            from blogs
            WHERE id = ?

        ").bind(id)
        .fetch_optional(&db)
        .await;

        match res {
            Ok(Some(blog)) => (StatusCode::OK, Json(Self {
                id: blog.id,
                title: blog.title,
                reads: blog.reads,
                body: blog.body
            })).into_response(),
            Ok(None) => (StatusCode::NOT_FOUND, format!("Todo with id {} not found", id)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response() 
        }
        
    }
    
    // update -> should be custom. custom code where everytime there is a fetch from db (i.e everytime the blog is read),
    // read increases by 1.


    pub async fn delete_a_blog(State(db): State<MySqlPool>, Path(id): Path<u32>) -> impl IntoResponse {
        let res = sqlx::query("
            DELETE FROM blogs
            WHERE id = ?
        ").bind(id)
        .execute(&db)
        .await;

        match res {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode:: INTERNAL_SERVER_ERROR
        }
    }

}