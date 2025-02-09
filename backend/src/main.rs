mod db;
mod models;
mod schema;
mod types;

use crate::db::connect;
use crate::models::User;
use crate::schema::users::dsl::users;
use crate::schema::users::name;
use crate::types::{ErrorResponse, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use argonautica::{Hasher, Verifier};
use axum::extract::rejection::JsonRejection;
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use diesel::expression_methods::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use serde_json::json;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenv().ok();

    info!("initializing router...");

    // Configure CORS to allow requests from frontend
    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/api/users", get(json_users))
        .route("/api/user/register", post(register))
        .route("/api/user/login", post(login))
        .layer(cors);

    info!("starting server...");

    let port = 3000_u16;
    let address = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();

    info!("server started on port {}", port);
}

async fn json_users() -> impl IntoResponse {
    let connection = &mut connect();
    let results = users
        .limit(10)
        .select(User::as_select())
        .load::<User>(connection)
        .expect("Error loading users");

    let mut json_users = Vec::new();
    for user in results {
        let json_user = json!({
            "id": user.id.to_string(),
            "name": user.name,
            "password": user.password,
            "elevated": user.elevated,
        });
        json_users.push(json_user);
    }

    (StatusCode::OK, Json(json!(json_users)))
}

/// Allows registering a new user.
///
/// This route accepts a JSON payload with the following structure:
/// - `username`: The name of the user to register.
/// - `password`: The password of the user to register.
pub async fn register(result: Result<Json<RegisterRequest>, JsonRejection>) -> impl IntoResponse {
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

/// Attempts to log in a user.
///
/// This route accepts a JSON payload with the following structure:
/// - `username`: The name of the user to log in.
/// - `password`: The password of the user to log in.
pub async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
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
