/// this module uses prkorm for sql queries
use axum::{extract::{State, Path}, http::StatusCode, response::IntoResponse, Json};
use sqlx::MySqlPool;
use chrono;

use super::*;

pub async fn create_new_user(State(db): State<MySqlPool>, Json(payload): Json<CreateUserType>) -> impl IntoResponse {
    let password = bcrypt::hash(payload.password, 12).unwrap();
    let query = User::insert().insert_to_email(payload.email).insert_to_username(payload.username).insert_to_password_hash(password).build();

    let response = sqlx::query(&query).execute(&db).await;

    match response {
        Ok(_) => Ok((StatusCode::CREATED, "User successfully created".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error creating user: {}", e.to_string())))
    }
}

pub async fn fetch_single_user(State(db): State<MySqlPool>, Path(id): Path<u32>) -> impl IntoResponse {
    let query = User::select().where_id(id).build();
    let response: Result<User, sqlx::Error> = sqlx::query_as(&query).fetch_one(&db).await;

    match response {
        Ok(user) => Ok((StatusCode::OK, Json(PublicUser::from(user)))),
        Err(_) => Err((StatusCode::NOT_FOUND, Json("User not found")))
    }
}

pub async fn get_users(State(db): State<MySqlPool>) -> impl IntoResponse {
    let query = User::select().build();
    let users: Result<Vec<User>, sqlx::Error> = sqlx::query_as(&query).fetch_all(&db).await;

    match users {
        Ok(users) => {
            let mut public_users = Vec::new();

            for user in users {
                let public_user = PublicUser::from(user);
                public_users.push(public_user);
            }

            (StatusCode::OK, Json(public_users))
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<PublicUser>::new()))
    }
}

pub async fn delete_user_by_id(State(db): State<MySqlPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let query = User::delete().delete_where_id_eq(id);
    let response = sqlx::query(&query).execute(&db).await;

    match response {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
}


// login with jwt
const SECRET: &[u8] = b"tomatopaste";

pub async fn login_user(State(db): State<MySqlPool>, Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    let query = User::select().where_email(&payload.email).build();
    let response: Result<Option<User>, sqlx::Error> = sqlx::query_as(&query).fetch_optional(&db).await;

    match response {
        Ok(Some(user)) => {
            let valid = bcrypt::verify(&payload.password, &user.password_hash);

            match valid {
                Ok(true) => {
                    let expiration = chrono::Utc::now()
                        .checked_add_signed(chrono::Duration::hours(24))
                        .unwrap()
                        .timestamp() as usize;

                    let claims = Claims {
                        sub: user.id,
                        exp: expiration,
                    };

                    let token = jsonwebtoken::encode(
                        &jsonwebtoken::Header::default(),
                        &claims,
                        &jsonwebtoken::EncodingKey::from_secret(SECRET),
                    )
                    .unwrap();        
                    let email = user.email.clone();
                    let username = user.username.clone(); 
                    return Ok((StatusCode::OK, Json(AuthorizedUser::from_input(token, email, username))));        
                }, 
                Ok(false) => {
                    Err((StatusCode::UNAUTHORIZED, "Invalid password".to_string()))
                },
                Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Unable to verify credentials".to_string()))
            }
        },
        Ok(None) => {
            return Err((StatusCode::UNAUTHORIZED, "This user does not exist".to_string()))
        },
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string()))
    }    

}

