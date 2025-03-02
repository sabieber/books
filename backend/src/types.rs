// types.rs
use serde::{Deserialize, Serialize};

/// Request type for registering a new user.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

/// Request type for logging in a user.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Request type for listing all shelves of a user.
#[derive(Debug, Deserialize)]
pub struct ShelvesRequest {
    pub user_id: String,
}

/// Request type for creating a new shelf.
#[derive(Debug, Deserialize)]
pub struct CreateShelfRequest {
    pub name: String,
    pub description: Option<String>,
    pub user_id: String,
}

/// Request type for adding a book to a shelf.
#[derive(Debug, Deserialize)]
pub struct AddBookToShelfRequest {
    pub user_id: String,
    pub shelf_id: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub isbn13: Option<String>,
    pub isbn10: Option<String>,
    pub google_books_id: Option<String>,
}

/// Request type for listing all books on a shelf.
#[derive(Debug, Deserialize)]
pub struct ShelfBooksRequest {
    pub shelf_id: String,
}

/// Request type for removing a book from a shelf.
#[derive(Debug, Deserialize)]
pub struct RemoveBookFromShelfRequest {
    pub book_id: String,
}

/// Request type for removing a shelf.
#[derive(Debug, Deserialize)]
pub struct RemoveShelfRequest {
    pub shelf_id: String,
}

/// Response type for listing all books on a shelf.
#[derive(Debug, Serialize)]
pub struct ShelfBooksResponse {
    pub books: Vec<serde_json::Value>,
}

/// Response type for a successful user registration.
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

/// Response type for a successful user login.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub user_id: String,
}

/// Response type for failed requests.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Request type for listing all shelves of a user.
#[derive(Debug, Serialize)]
pub struct ListShelvesResponse {
    pub shelves: Vec<serde_json::Value>,
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

/// Request type for getting information about a reading session.
#[derive(Debug, Deserialize)]
pub struct ReadingInfoRequest {
    pub reading_id: String,
}

/// Request type for starting a new reading session.
#[derive(Debug, Deserialize)]
pub struct StartReadingRequest {
    pub book_id: String,
    pub user_id: String,
    pub total_pages: i32,
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
