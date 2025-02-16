// types.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserIdRequest {
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateShelfRequest {
    pub name: String,
    pub description: Option<String>,
    pub user_id: String,
}

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

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub user_id: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct ListShelvesResponse {
    pub shelves: Vec<serde_json::Value>,
}
