use crate::db::connect;
use crate::models::{Reading, ReadingEntry, ReadingMode};
use crate::schema::reading_entries::dsl::reading_entries;
use crate::schema::readings::dsl::readings;
use crate::{schema, ErrorResponse};
use axum::routing::post;
use axum::{extract::Json, http::StatusCode, response::IntoResponse, Router};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

pub(crate) fn register_routes(router: Router) -> Router {
    router
        .route("/api/books/reading", post(get_reading_info))
        .route("/api/books/start-reading", post(start_reading_session))
        .route("/api/books/track-progress", post(track_progress))
}

/// Request type for getting information about a reading session.
#[derive(Debug, Deserialize)]
pub struct ReadingInfoRequest {
    pub reading_id: String,
}

/// Fetches reading session information by reading ID.
///
/// This route accepts a JSON payload with the following structure:
/// - `reading_id`: The UUID of the reading session to fetch information for.
pub(crate) async fn get_reading_info(Json(payload): Json<ReadingInfoRequest>) -> impl IntoResponse {
    let connection = &mut connect();

    let reading_id = Uuid::parse_str(&payload.reading_id).expect("Invalid reading ID");

    let db_entries = reading_entries
        .filter(crate::schema::reading_entries::dsl::reading.eq(reading_id))
        .load::<ReadingEntry>(connection)
        .expect("Error loading entries");

    let mut json_entries = Vec::new();
    for entry in db_entries {
        let json_entry = json!({
            "id": entry.id.to_string(),
            "progress": entry.progress,
            "mode": entry.mode.to_string(),
            "read_at": entry.read_at.to_string(),
        });
        json_entries.push(json_entry);
    }

    match readings
        .filter(schema::readings::dsl::id.eq(reading_id))
        .first::<Reading>(connection)
    {
        Ok(reading) => (
            StatusCode::OK,
            Json(json!(json!({
                "book_id": reading.book.to_string(),
                "entries": json_entries,
            }))),
        ),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!(ErrorResponse {
                error: "Reading not found.".to_string(),
            })),
        ),
    }
}

/// Request type for starting a new reading session.
#[derive(Debug, Deserialize)]
pub struct StartReadingRequest {
    pub book_id: String,
    pub user_id: String,
    pub total_pages: i32,
}

/// Starts a new reading session for a book.
///
/// This route accepts a JSON payload with the following structure:
/// - `book_id`: The UUID of the book to start reading.
/// - `user_id`: The UUID of the user starting the reading session.
/// - `total_pages`: The total number of pages of the book.
pub(crate) async fn start_reading_session(
    Json(payload): Json<StartReadingRequest>,
) -> impl IntoResponse {
    let connection = &mut connect();

    let book_id = Uuid::parse_str(&payload.book_id).expect("Invalid book ID");
    let user_id = Uuid::parse_str(&payload.user_id).expect("Invalid user ID");

    let new_reading = Reading {
        id: Uuid::new_v4(),
        book: book_id,
        user: user_id,
        total_pages: payload.total_pages,
        progress: 0,
        mode: ReadingMode::Pages,
        started_at: chrono::Utc::now().date_naive(),
        finished_at: None,
        cancelled_at: None,
        updated_at: chrono::Utc::now().naive_utc(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    match diesel::insert_into(schema::readings::dsl::readings)
        .values(&new_reading)
        .execute(connection)
    {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Reading session started successfully." })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse {
                error: format!("Error while starting the reading session: {}", e),
            })),
        ),
    }
}

/// Request type for tracking progress in a reading session.
#[derive(Debug, Deserialize)]
pub struct TrackProgressRequest {
    pub reading_id: String,
    pub book_id: String,
    pub user_id: String,
    pub progress: i32,
    pub read_at: String,
}

/// Tracks progress for a reading session.
///
/// This route accepts a JSON payload with the following structure:
/// - `reading_id`: The UUID of the reading session.
/// - `user_id`: The UUID of the user tracking the progress.
/// - `progress`: The page number reached.
/// - `read_at`: The date when reading took place.
pub(crate) async fn track_progress(Json(payload): Json<TrackProgressRequest>) -> impl IntoResponse {
    let connection = &mut connect();

    let reading_id = Uuid::parse_str(&payload.reading_id).expect("Invalid reading ID");
    let book_id = Uuid::parse_str(&payload.book_id).expect("Invalid book ID");
    let user_id = Uuid::parse_str(&payload.user_id).expect("Invalid user ID");

    let new_entry = ReadingEntry {
        id: Uuid::new_v4(),
        reading: reading_id,
        book: book_id,
        user: user_id,
        progress: payload.progress,
        mode: ReadingMode::Pages,
        read_at: chrono::NaiveDate::parse_from_str(&payload.read_at, "%Y-%m-%d")
            .expect("Invalid date format"),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let transaction_result = connection.transaction::<_, diesel::result::Error, _>(|connection| {
        diesel::insert_into(reading_entries)
            .values(&new_entry)
            .execute(connection)?;

        diesel::update(readings.filter(schema::readings::dsl::id.eq(reading_id)))
            .set(schema::readings::dsl::progress.eq(payload.progress))
            .execute(connection)?;

        Ok(())
    });

    match transaction_result {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Progress tracked successfully." })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse {
                error: format!("Error while tracking progress: {}", e),
            })),
        ),
    }
}
