mod db;
mod models;
mod schema;
mod types;

use crate::db::connect;
use crate::models::{Book, Reading, ReadingEntry, ReadingMode, Shelf, User};
use crate::schema::books::dsl::books;
use crate::schema::reading_entries::dsl::reading_entries;
use crate::schema::readings::dsl::readings;
use crate::schema::users::dsl::users;
use crate::schema::users::name;
use crate::types::{
    AddBookToShelfRequest, BookInfoRequest, BookInfoResponse, CreateShelfRequest, ErrorResponse,
    ListShelvesResponse, LoginRequest, LoginResponse, ReadingInfoRequest, RegisterRequest,
    RegisterResponse, RemoveBookFromShelfRequest, RemoveShelfRequest, ShelfBooksRequest,
    ShelvesRequest, StartReadingRequest, TrackProgressRequest,
};
use argonautica::{Hasher, Verifier};
use axum::extract::rejection::JsonRejection;
use axum::{extract::Json, http::StatusCode, response::IntoResponse, routing::post, Router};
use diesel::expression_methods::ExpressionMethods;
use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use serde_json::json;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use uuid::Uuid;

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

    let router = Router::new()
        .route("/api/user/register", post(register))
        .route("/api/user/login", post(login))
        .route("/api/shelves", post(list_shelves))
        .route("/api/shelves/create", post(create_shelf))
        .route("/api/shelves/add-book", post(add_book_to_shelf))
        .route("/api/shelves/books", post(list_shelf_books))
        .route("/api/shelves/remove-book", post(remove_book_from_shelf))
        .route("/api/shelves/remove", post(remove_shelf))
        .route("/api/books/info", post(get_book_info))
        .route("/api/books/reading", post(get_reading_info))
        .route("/api/books/start-reading", post(start_reading_session))
        .route("/api/books/track-progress", post(track_progress))
        .layer(cors);

    info!("starting server...");

    let port = 3000_u16;
    let address = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();

    info!("server started on port {}", port);
}

/// Allows registering a new user.
///
/// This route accepts a JSON payload with the following structure:
/// - `username`: The name of the user to register.
/// - `password`: The password of the user to register.
async fn register(result: Result<Json<RegisterRequest>, JsonRejection>) -> impl IntoResponse {
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
async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
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

/// Lists the shelves of a user.
///
/// This route accepts a query parameter `user_id` which is the UUID of the user.
async fn list_shelves(Json(payload): Json<ShelvesRequest>) -> impl IntoResponse {
    let connection = &mut connect();

    let user_id = Uuid::parse_str(&payload.user_id).expect("Invalid user ID");
    let results = crate::schema::shelves::dsl::shelves
        .filter(crate::schema::shelves::dsl::user.eq(user_id))
        .load::<Shelf>(connection)
        .expect("Error loading shelves");

    let mut json_shelves = Vec::new();
    for shelf in results {
        let json_shelf = json!({
            "id": shelf.id.to_string(),
            "name": shelf.name,
            "description": shelf.description,
            "user": shelf.user.to_string(),
            "created_at": shelf.created_at.to_string(),
            "updated_at": shelf.updated_at.to_string(),
        });
        json_shelves.push(json_shelf);
    }

    (
        StatusCode::OK,
        Json(json!(ListShelvesResponse {
            shelves: json_shelves,
        })),
    )
}

/// Creates a new shelf.
///
/// This route accepts a JSON payload with the following structure:
/// - `name`: The name of the shelf.
/// - `description`: The description of the shelf.
/// - `user_id`: The UUID of the user who owns the shelf.
async fn create_shelf(Json(payload): Json<CreateShelfRequest>) -> impl IntoResponse {
    let new_shelf = Shelf {
        id: Uuid::new_v4(),
        name: payload.name.trim().to_string(),
        description: payload.description.map(|d| d.trim().to_string()),
        user: Uuid::parse_str(&payload.user_id).expect("Invalid user ID"),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let connection = &mut connect();

    match diesel::insert_into(crate::schema::shelves::dsl::shelves)
        .values(&new_shelf)
        .execute(connection)
    {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Shelf created successfully." })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse {
                error: format!("Error while creating the shelf: {}", e),
            })),
        ),
    }
}

/// Removes a shelf.
///
/// This route accepts a JSON payload with the following structure:
/// - `shelf_id`: The UUID of the shelf to remove.
async fn remove_shelf(Json(payload): Json<RemoveShelfRequest>) -> impl IntoResponse {
    let connection = &mut connect();

    let shelf_id = Uuid::parse_str(&payload.shelf_id).expect("Invalid shelf ID");

    let delete_books_result = diesel::delete(
        crate::schema::books::dsl::books.filter(crate::schema::books::dsl::shelf.eq(shelf_id)),
    )
    .execute(connection);

    if let Err(e) = delete_books_result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse {
                error: format!("Error while removing books from the shelf: {}", e),
            })),
        );
    }

    match diesel::delete(
        crate::schema::shelves::dsl::shelves.filter(crate::schema::shelves::dsl::id.eq(shelf_id)),
    )
    .execute(connection)
    {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Shelf and its books removed successfully." })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse {
                error: format!("Error while removing the shelf: {}", e),
            })),
        ),
    }
}

/// Lists the books of a shelf.
///
/// This route accepts a JSON payload with the following structure:
/// - `shelf_id`: The UUID of the shelf to list the books of.
async fn list_shelf_books(Json(payload): Json<ShelfBooksRequest>) -> impl IntoResponse {
    let connection = &mut connect();

    let shelf_id = Uuid::parse_str(&payload.shelf_id).expect("Invalid shelf ID");
    let shelf = crate::schema::shelves::dsl::shelves
        .filter(crate::schema::shelves::dsl::id.eq(shelf_id))
        .first::<Shelf>(connection)
        .expect("Error loading shelf");

    let results = books
        .filter(crate::schema::books::dsl::shelf.eq(shelf_id))
        .load::<Book>(connection)
        .expect("Error loading books");

    let mut json_books = Vec::new();
    for book in results {
        let json_book = json!({
            "id": book.id.to_string(),
            "title": book.title,
            "author": book.author,
            "isbn13": book.isbn13,
            "isbn10": book.isbn10,
            "google_books_id": book.google_books_id,
            "added_at": book.added_at.to_string(),
        });
        json_books.push(json_book);
    }

    (
        StatusCode::OK,
        Json(json!({
            "shelf": {
                "id": shelf.id.to_string(),
                "name": shelf.name,
                "description": shelf.description,
                "user": shelf.user.to_string(),
                "created_at": shelf.created_at.to_string(),
                "updated_at": shelf.updated_at.to_string(),
            },
            "books": json_books,
        })),
    )
}

/// Adds a book to a shelf.
///
/// This route accepts a JSON payload with the following structure:
/// - `user_id`: The UUID of the user who owns the shelf.
/// - `shelf_id`: The UUID of the shelf to add the book to.
/// - `title`: The title of the book.
/// - `author`: The author of the book.
/// - `isbn13`: The ISBN-13 of the book.
/// - `isbn10`: The ISBN-10 of the book.
/// - `google_books_id`: The Google Books ID of the book.
async fn add_book_to_shelf(Json(payload): Json<AddBookToShelfRequest>) -> impl IntoResponse {
    let connection = &mut connect();

    let new_book = Book {
        id: Uuid::new_v4(),
        user: Uuid::parse_str(&payload.user_id).expect("Invalid user ID"),
        shelf: Uuid::parse_str(&payload.shelf_id).expect("Invalid shelf ID"),
        title: payload.title,
        author: payload.author,
        isbn13: payload.isbn13,
        isbn10: payload.isbn10,
        google_books_id: payload.google_books_id,
        added_at: chrono::Utc::now().naive_utc(),
    };

    match diesel::insert_into(schema::books::dsl::books)
        .values(&new_book)
        .execute(connection)
    {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Book added to shelf successfully." })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse {
                error: format!("Error while adding the book to the shelf: {}", e),
            })),
        ),
    }
}

/// Removes a book from a shelf.
///
/// This route accepts a JSON payload with the following structure:
/// - `book_id`: The UUID of the book to remove from the shelf.
async fn remove_book_from_shelf(
    Json(payload): Json<RemoveBookFromShelfRequest>,
) -> impl IntoResponse {
    let connection = &mut connect();

    let book_id = Uuid::parse_str(&payload.book_id).expect("Invalid book ID");

    match diesel::delete(
        crate::schema::books::dsl::books.filter(crate::schema::books::dsl::id.eq(book_id)),
    )
    .execute(connection)
    {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Book removed from shelf successfully." })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse {
                error: format!("Error while removing the book from the shelf: {}", e),
            })),
        ),
    }
}

/// Fetches book information by book ID.
///
/// This route accepts a JSON payload with the following structure:
/// - `book_id`: The UUID of the book to fetch information for.
async fn get_book_info(Json(payload): Json<BookInfoRequest>) -> impl IntoResponse {
    let connection = &mut connect();

    let book_id = Uuid::parse_str(&payload.book_id).expect("Invalid book ID");

    let db_readings = readings
        .filter(crate::schema::readings::dsl::book.eq(book_id))
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

/// Fetches reading session information by reading ID.
///
/// This route accepts a JSON payload with the following structure:
/// - `reading_id`: The UUID of the reading session to fetch information for.
async fn get_reading_info(Json(payload): Json<ReadingInfoRequest>) -> impl IntoResponse {
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

/// Starts a new reading session for a book.
///
/// This route accepts a JSON payload with the following structure:
/// - `book_id`: The UUID of the book to start reading.
/// - `user_id`: The UUID of the user starting the reading session.
/// - `total_pages`: The total number of pages of the book.
async fn start_reading_session(Json(payload): Json<StartReadingRequest>) -> impl IntoResponse {
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

/// Tracks progress for a reading session.
///
/// This route accepts a JSON payload with the following structure:
/// - `reading_id`: The UUID of the reading session.
/// - `user_id`: The UUID of the user tracking the progress.
/// - `progress`: The page number reached.
/// - `read_at`: The date when reading took place.
async fn track_progress(Json(payload): Json<TrackProgressRequest>) -> impl IntoResponse {
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
