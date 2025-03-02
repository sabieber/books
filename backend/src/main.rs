mod books;
mod db;
mod models;
mod readings;
mod schema;
mod shelves;
mod users;

use axum::Router;
use dotenvy::dotenv;
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

/// Main entry point for the application and basic setup of the web server.
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

    let mut router = Router::new();
    router = users::register_routes(router);
    router = shelves::register_routes(router);
    router = books::register_routes(router);
    router = readings::register_routes(router);
    router = router.layer(cors);

    info!("starting server...");

    let port = 3000_u16;
    let address = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();

    info!("server started on port {}", port);
}

/// Response type for failed requests.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
