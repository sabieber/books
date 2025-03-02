use crate::db::connect;
use crate::models::User;
use crate::schema::users::dsl::users;
use crate::schema::users::name;
use crate::ErrorResponse;
use argonautica::{Hasher, Verifier};
use axum::extract::rejection::JsonRejection;
use axum::routing::post;
use axum::{extract::Json, http::StatusCode, response::IntoResponse, Router};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use uuid::Uuid;

pub(crate) fn register_routes(router: Router) -> Router {
    router
        .route("/api/user/register", post(register))
        .route("/api/user/login", post(login))
}

/// Request type for registering a new user.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

/// Response type for a successful user registration.
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

/// Allows registering a new user.
///
/// This route accepts a JSON payload with the following structure:
/// - `username`: The name of the user to register.
/// - `password`: The password of the user to register.
pub(crate) async fn register(
    result: Result<Json<RegisterRequest>, JsonRejection>,
) -> impl IntoResponse {
    let payload = match result {
        Ok(payload) => payload,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!(ErrorResponse {
                    error: err.body_text()
                })),
            )
        }
    };

    let secret = env::var("PASSWORD_SECRET").expect("PASSWORD_SECRET must be set");

    let mut hasher = Hasher::default();
    let password_hash = hasher
        .with_password(payload.password.clone())
        .with_secret_key(secret)
        .hash()
        .unwrap();

    let new_user = User {
        id: Uuid::new_v4(),
        name: payload.username.clone(),
        password: password_hash,
        elevated: false,
    };

    let connection = &mut connect();

    match diesel::insert_into(users)
        .values(&new_user)
        .execute(connection)
    {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!(RegisterResponse {
                message: "Successfully registered user.".to_string(),
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse {
                error: format!("Error while registering the user: {}", e),
            })),
        ),
    }
}

/// Request type for logging in a user.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Response type for a successful user login.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub user_id: String,
}

/// Attempts to log in a user.
///
/// This route accepts a JSON payload with the following structure:
/// - `username`: The name of the user to log in.
/// - `password`: The password of the user to log in.
pub(crate) async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    let secret = env::var("PASSWORD_SECRET").expect("PASSWORD_SECRET must be set");

    let connection = &mut connect();

    let user: User = match users.filter(name.eq(&payload.username)).first(connection) {
        Ok(user) => user,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!(ErrorResponse {
                    error: "Login failed.".to_string(),
                })),
            )
        }
    };

    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(user.password)
        .with_password(payload.password)
        .with_secret_key(secret)
        .verify()
        .unwrap();

    if is_valid {
        (
            StatusCode::OK,
            Json(json!(LoginResponse {
                message: "Successfully logged in user.".to_string(),
                user_id: user.id.to_string(),
            })),
        )
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!(ErrorResponse {
                error: "Login failed.".to_string(),
            })),
        )
    }
}
