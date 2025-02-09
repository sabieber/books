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

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
