use axum::{response::IntoResponse, Router};
use axum::routing::{delete, get, post};
use axum::extract::{Path, Json};

use serde::{Serialize, Deserialize};
use tokio::net::TcpListener;

mod database;
use database::*;

mod blog;
use blog::*;

#[tokio::main]
async fn main() {

    let db = database_connection().await.expect("Failed to connect to Database");


    let routes = Router::new()
                            .route("/", get(handle_home_request))
                            .route("/user/{id}/fetch", get(fetch_single_user))
                            .route("/user/create", post(create_new_user))
                            .route("/blog/new", post(Blog::create_a_blog))
                            .route("/blogs/all", get(Blog::fetch_all_blogs))
                            .route("/blog/{id}", get(Blog::fetch_single_blog))
                            .route("/blog/{id}/delete", delete(Blog::delete_a_blog))
                            .with_state(db); 

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Server running on localhost:8000");

    axum::serve(listener, routes.into_make_service()).await.unwrap();    
    
}

#[derive(Serialize)]
pub struct User<'a> {
    id: u32,
    name: &'a str
}

#[derive(Deserialize)]
pub struct CreateUserType {
    id: u32,
    name: String
}


// todo: add user to db and make create user more functional

pub async fn handle_home_request() -> impl IntoResponse {
    "Hello, welcome to our useful Blog!\n"
}

pub async fn fetch_single_user(Path(id): Path<u32>) -> impl IntoResponse {
    Json(User{
        id,
        name: "Fuad Olaleye"
    })
}

pub async fn create_new_user(Json(user): Json<CreateUserType>) -> impl IntoResponse {
    Json(User{
        id: user.id,
        name: "Fuad Olaleye"
    })
}