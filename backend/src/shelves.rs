use crate::db::connect;
use crate::models::{Book, Shelf};
use crate::schema::books::dsl::books;
use crate::{schema, ErrorResponse};
use axum::routing::post;
use axum::{extract::Json, http::StatusCode, response::IntoResponse, Router};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

pub(crate) fn register_routes(router: Router) -> Router {
    router
        .route("/api/shelves", post(list_shelves))
        .route("/api/shelves/create", post(create_shelf))
        .route("/api/shelves/add-book", post(add_book_to_shelf))
        .route("/api/shelves/books", post(list_shelf_books))
        .route("/api/shelves/remove-book", post(remove_book_from_shelf))
        .route("/api/shelves/remove", post(remove_shelf))
}

/// Request type for listing all shelves of a user.
#[derive(Debug, Deserialize)]
pub struct ShelvesRequest {
    pub user_id: String,
}

/// Request type for listing all shelves of a user.
#[derive(Debug, Serialize)]
pub struct ListShelvesResponse {
    pub shelves: Vec<serde_json::Value>,
}

/// Lists the shelves of a user.
///
/// This route accepts a query parameter `user_id` which is the UUID of the user.
pub(crate) async fn list_shelves(Json(payload): Json<ShelvesRequest>) -> impl IntoResponse {
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

/// Request type for creating a new shelf.
#[derive(Debug, Deserialize)]
pub struct CreateShelfRequest {
    pub name: String,
    pub description: Option<String>,
    pub user_id: String,
}

/// Creates a new shelf.
///
/// This route accepts a JSON payload with the following structure:
/// - `name`: The name of the shelf.
/// - `description`: The description of the shelf.
/// - `user_id`: The UUID of the user who owns the shelf.
pub(crate) async fn create_shelf(Json(payload): Json<CreateShelfRequest>) -> impl IntoResponse {
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

/// Request type for removing a shelf.
#[derive(Debug, Deserialize)]
pub struct RemoveShelfRequest {
    pub shelf_id: String,
}

/// Removes a shelf.
///
/// This route accepts a JSON payload with the following structure:
/// - `shelf_id`: The UUID of the shelf to remove.
pub(crate) async fn remove_shelf(Json(payload): Json<RemoveShelfRequest>) -> impl IntoResponse {
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

/// Request type for listing all books on a shelf.
#[derive(Debug, Deserialize)]
pub struct ShelfBooksRequest {
    pub shelf_id: String,
}

/// Lists the books of a shelf.
///
/// This route accepts a JSON payload with the following structure:
/// - `shelf_id`: The UUID of the shelf to list the books of.
pub(crate) async fn list_shelf_books(Json(payload): Json<ShelfBooksRequest>) -> impl IntoResponse {
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
pub(crate) async fn add_book_to_shelf(
    Json(payload): Json<AddBookToShelfRequest>,
) -> impl IntoResponse {
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

/// Request type for removing a book from a shelf.
#[derive(Debug, Deserialize)]
pub struct RemoveBookFromShelfRequest {
    pub book_id: String,
}

/// Removes a book from a shelf.
///
/// This route accepts a JSON payload with the following structure:
/// - `book_id`: The UUID of the book to remove from the shelf.
pub(crate) async fn remove_book_from_shelf(
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
