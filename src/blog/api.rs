use axum::{extract::{Path, State}, http::StatusCode, Json, response::IntoResponse};
use sqlx::MySqlPool;
use super::*;

pub async fn create_a_blog(State(db): State<MySqlPool>, Json(body): Json<CreateBlog>) -> impl IntoResponse { // bit of sql here since we are writing to the database
    let query = Blog::insert().insert_to_body(&body.body).insert_to_title(&body.title).insert_to_reads_count(0).insert_to_user_id(&body.user_id).build();
    let res = sqlx::query(&query).execute(&db).await;

    match res {
        Ok(blog) => (StatusCode::CREATED, Json(Blog{
            id: blog.last_insert_id() as i32,
            user_id: body.user_id,
            body: body.body.clone(),
            reads_count: 0,
            title: body.title.clone()
        })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

pub async fn fetch_all_blogs(State(db): State<MySqlPool>) -> impl IntoResponse {
    let res: Result<Vec<Blog>, sqlx::Error> = sqlx::query_as(
        "
        SELECT id, user_id, title, reads_count, body, created_at
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
        SET reads_count =  reads_count + 1
        WHERE id = ?
        "
    ).bind(id)
    .execute(&db)
    .await.unwrap();


    let res = sqlx::query_as::<_, Blog>("
        SELECT id, user_id, title, body, reads_count
        from blogs
        WHERE id = ?

    ").bind(id)
    .fetch_optional(&db)
    .await;

    match res {
        Ok(Some(blog)) => (StatusCode::OK, Json(Blog {
            id: blog.id,
            user_id: blog.user_id,
            title: blog.title,
            reads_count: blog.reads_count,
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

pub async fn get_all_blogs_by_user_id(State(db): State<MySqlPool>, Path(user_id): Path<i32>) -> impl IntoResponse {
    let query = Blog::select().where_user_id(user_id).build();
    let response: Result<Vec<Blog>, sqlx::Error> = sqlx::query_as(&query).fetch_all(&db).await;

    match response {
        Ok(blogs) => Ok((StatusCode::OK, Json(blogs))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Error fetching blogs: {}", e.to_string()))))
    }
}
