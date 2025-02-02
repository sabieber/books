mod db;
mod models;
mod schema;

use crate::models::User;
use crate::schema::users::dsl::users;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Json, Router};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use serde_json::json;
use tracing::info;
use crate::db::connect;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    info!("initializing router...");

    let router = Router::new()
        .route("/api/users", get(json_users));

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
            "salt": user.salt,
            "elevated": user.elevated,
        });
        json_users.push(json_user);
    }

    (StatusCode::OK, Json(json!(json_users)))
}
