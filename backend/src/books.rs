use crate::db::connect;
use crate::models::{Book, Reading};
use crate::schema::books::dsl::books;
use crate::schema::readings::dsl::readings;
use crate::{schema, ErrorResponse};
use axum::routing::post;
use axum::{extract::Json, http::StatusCode, response::IntoResponse, Router};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

pub(crate) fn register_routes(router: Router) -> Router {
    router.route("/api/books/info", post(get_book_info))
}

/// Request type for getting information about a book.
#[derive(Debug, Deserialize)]
pub struct BookInfoRequest {
    pub book_id: String,
}

/// Response type for book information.
#[derive(Debug, Serialize)]
pub struct BookInfoResponse {
    pub google_books_id: Option<String>,
    pub readings: Vec<serde_json::Value>,
}

/// Fetches book information by book ID.
///
/// This route accepts a JSON payload with the following structure:
/// - `book_id`: The UUID of the book to fetch information for.
pub(crate) async fn get_book_info(Json(payload): Json<BookInfoRequest>) -> impl IntoResponse {
    let connection = &mut connect();

    let book_id = Uuid::parse_str(&payload.book_id).expect("Invalid book ID");

    let db_readings = readings
        .filter(schema::readings::dsl::book.eq(book_id))
        .load::<Reading>(connection)
        .expect("Error loading readings");

    let mut json_readings = Vec::new();
    for reading in db_readings {
        let json_reading = json!({
            "id": reading.id.to_string(),
            "total_pages": reading.total_pages,
            "progress": reading.progress,
            "mode": reading.mode.to_string(),
            "started_at": reading.started_at.to_string(),
            "finished_at": reading.finished_at.map(|d| d.to_string()),
            "cancelled_at": reading.cancelled_at.map(|d| d.to_string()),
        });
        json_readings.push(json_reading);
    }

    match books
        .filter(schema::books::dsl::id.eq(book_id))
        .first::<Book>(connection)
    {
        Ok(book) => (
            StatusCode::OK,
            Json(json!(BookInfoResponse {
                google_books_id: book.google_books_id,
                readings: json_readings,
            })),
        ),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!(ErrorResponse {
                error: "Book not found.".to_string(),
            })),
        ),
    }
}
