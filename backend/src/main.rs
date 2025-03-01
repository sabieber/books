mod db;
mod models;
mod schema;
mod types;

use crate::db::connect;
use crate::models::{Shelf, User, Book};
use crate::schema::users::dsl::users;
use crate::schema::users::name;
use crate::schema::books::dsl::books;
use crate::types::{ErrorResponse, ListShelvesResponse, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, UserIdRequest, CreateShelfRequest, AddBookToShelfRequest, ShelfBooksRequest, RemoveBookFromShelfRequest, RemoveShelfRequest, BookInfoResponse, BookInfoRequest};
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
        .route("/api/shelves", post(list_shelves))
        .route("/api/shelves/create", post(create_shelf))
        .route("/api/shelves/add-book", post(add_book_to_shelf))
        .route("/api/shelves/books", post(list_shelf_books))
        .route("/api/shelves/remove-book", post(remove_book_from_shelf))
        .route("/api/shelves/remove", post(remove_shelf))
        .route("/api/books/info", post(get_book_info))
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

/// Lists the shelves of a user.
///
/// This route accepts a query parameter `user_id` which is the UUID of the user.
pub async fn list_shelves(Json(payload): Json<UserIdRequest>) -> impl IntoResponse {
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

    (StatusCode::OK, Json(json!(ListShelvesResponse {
        shelves: json_shelves,
    })))
}

/// Creates a new shelf.
///
/// This route accepts a JSON payload with the following structure:
/// - `name`: The name of the shelf.
/// - `description`: The description of the shelf.
/// - `user_id`: The UUID of the user who owns the shelf.
pub async fn create_shelf(Json(payload): Json<CreateShelfRequest>) -> impl IntoResponse {
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
pub async fn remove_shelf(Json(payload): Json<RemoveShelfRequest>) -> impl IntoResponse {
    let connection = &mut connect();

    let shelf_id = Uuid::parse_str(&payload.shelf_id).expect("Invalid shelf ID");

    let delete_books_result = diesel::delete(crate::schema::books::dsl::books.filter(crate::schema::books::dsl::shelf.eq(shelf_id)))
        .execute(connection);

    if let Err(e) = delete_books_result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse {
                error: format!("Error while removing books from the shelf: {}", e),
            })),
        );
    }

    match diesel::delete(crate::schema::shelves::dsl::shelves.filter(crate::schema::shelves::dsl::id.eq(shelf_id)))
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
pub async fn list_shelf_books(Json(payload): Json<ShelfBooksRequest>) -> impl IntoResponse {
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

    (StatusCode::OK, Json(json!({
        "shelf": {
            "id": shelf.id.to_string(),
            "name": shelf.name,
            "description": shelf.description,
            "user": shelf.user.to_string(),
            "created_at": shelf.created_at.to_string(),
            "updated_at": shelf.updated_at.to_string(),
        },
        "books": json_books,
    })))
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
pub async fn remove_book_from_shelf(Json(payload): Json<RemoveBookFromShelfRequest>) -> impl IntoResponse {
    let connection = &mut connect();

    let book_id = Uuid::parse_str(&payload.book_id).expect("Invalid book ID");

    match diesel::delete(crate::schema::books::dsl::books.filter(crate::schema::books::dsl::id.eq(book_id)))
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
pub async fn get_book_info(Json(payload): Json<BookInfoRequest>) -> impl IntoResponse {
    let connection = &mut connect();

    let book_id = Uuid::parse_str(&payload.book_id).expect("Invalid book ID");

    match books.filter(schema::books::dsl::id.eq(book_id)).first::<Book>(connection) {
        Ok(book) => (
            StatusCode::OK,
            Json(json!(BookInfoResponse {
                google_books_id: book.google_books_id,
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
