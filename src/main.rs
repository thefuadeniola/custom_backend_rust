use axum::{response::IntoResponse, Router};
use axum::routing::{delete, get, post};
use tokio::net::TcpListener;

mod database;
use database::*;

mod blog;
use blog::*;

mod user;
use  user::*;

#[tokio::main]
async fn main() {

    let db = database_connection().await.expect("Failed to connect to Database");


    let routes = Router::new()
                            .route("/", get(handle_home_request))
                            .route("/user/{id}/fetch", get(api::fetch_single_user))
                            .route("/user/create", post(api::create_new_user))
                            .route("/users", get(api::get_users))
                            .route("/user/login", post(api::login_user))
                            .route("/user/delete/{id}", delete(api::delete_user_by_id))
                            .route("/blog/new", post(Blog::create_a_blog))
                            .route("/blogs/all", get(Blog::fetch_all_blogs))
                            .route("/blog/{id}", get(Blog::fetch_single_blog))
                            .route("/blog/{id}/delete", delete(Blog::delete_a_blog))
                            .with_state(db); 

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Server running on localhost:8000");

    axum::serve(listener, routes.into_make_service()).await.unwrap();    
    
}



// todo: add user to db and make create user more functional

pub async fn handle_home_request() -> impl IntoResponse {
    "Hello, welcome to our useful Blog!\n"
}

