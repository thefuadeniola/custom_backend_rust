use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;
use prkorm::*;

#[derive(Table, Serialize, Deserialize, Debug, FromRow)]
#[table_name("users")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserType {
    pub username: String,
    pub email: String,
    pub password: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String
}

impl From<User> for PublicUser {
    fn from(value: User) -> Self {
        Self { id: value.id, username: value.username, email: value.email }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims { // for login
    pub sub: i32,
    pub exp: usize
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizedUser {
    pub token: String,
    pub username: String,
    pub email: String
}

impl AuthorizedUser {
    pub fn from_input(token: String, email: String, username: String) -> Self {
        Self {
            token, email, username
        }
    }
}
