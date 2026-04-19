use crate::auth::AuthUser;
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
#[derive(Debug, Serialize)]
pub struct ListShelvesResponse {
    pub shelves: Vec<serde_json::Value>,
}

/// Lists the shelves of a user.
pub(crate) async fn list_shelves(auth: AuthUser) -> impl IntoResponse {
    let connection = &mut connect();
    let user_id = auth.0;

    let results = match crate::schema::shelves::dsl::shelves
        .filter(crate::schema::shelves::dsl::user.eq(user_id))
        .load::<Shelf>(connection)
    {
        Ok(r) => r,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse { error: format!("Error loading shelves: {}", e) })),
        ),
    };

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

    (StatusCode::OK, Json(json!(ListShelvesResponse { shelves: json_shelves })))
}

/// Request type for creating a new shelf.
#[derive(Debug, Deserialize)]
pub struct CreateShelfRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Creates a new shelf.
pub(crate) async fn create_shelf(
    auth: AuthUser,
    Json(payload): Json<CreateShelfRequest>,
) -> impl IntoResponse {
    let new_shelf = Shelf {
        id: Uuid::new_v4(),
        name: payload.name.trim().to_string(),
        description: payload.description.map(|d| d.trim().to_string()),
        user: auth.0,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let connection = &mut connect();

    match diesel::insert_into(crate::schema::shelves::dsl::shelves)
        .values(&new_shelf)
        .execute(connection)
    {
        Ok(_) => (StatusCode::CREATED, Json(json!({ "message": "Shelf created successfully." }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse { error: format!("Error while creating the shelf: {}", e) })),
        ),
    }
}

/// Request type for removing a shelf.
#[derive(Debug, Deserialize)]
pub struct RemoveShelfRequest {
    pub shelf_id: String,
}

/// Removes a shelf.
pub(crate) async fn remove_shelf(
    auth: AuthUser,
    Json(payload): Json<RemoveShelfRequest>,
) -> impl IntoResponse {
    let connection = &mut connect();
    let shelf_id = match Uuid::parse_str(&payload.shelf_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!(ErrorResponse { error: "Invalid shelf ID.".to_string() }))),
    };

    let shelf: Shelf = match crate::schema::shelves::dsl::shelves
        .filter(crate::schema::shelves::dsl::id.eq(shelf_id))
        .first(connection)
    {
        Ok(s) => s,
        Err(_) => return (StatusCode::NOT_FOUND, Json(json!(ErrorResponse { error: "Shelf not found.".to_string() }))),
    };

    if shelf.user != auth.0 {
        return (StatusCode::FORBIDDEN, Json(json!(ErrorResponse { error: "Access denied.".to_string() })));
    }

    let result = connection.transaction::<_, diesel::result::Error, _>(|conn| {
        diesel::delete(
            crate::schema::books::dsl::books.filter(crate::schema::books::dsl::shelf.eq(shelf_id)),
        )
        .execute(conn)?;

        diesel::delete(
            crate::schema::shelves::dsl::shelves.filter(crate::schema::shelves::dsl::id.eq(shelf_id)),
        )
        .execute(conn)?;

        Ok(())
    });

    match result {
        Ok(_) => (StatusCode::OK, Json(json!({ "message": "Shelf and its books removed successfully." }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse { error: format!("Error while removing the shelf: {}", e) })),
        ),
    }
}

/// Request type for listing all books on a shelf.
#[derive(Debug, Deserialize)]
pub struct ShelfBooksRequest {
    pub shelf_id: String,
}

/// Lists the books of a shelf.
pub(crate) async fn list_shelf_books(
    auth: AuthUser,
    Json(payload): Json<ShelfBooksRequest>,
) -> impl IntoResponse {
    let connection = &mut connect();

    let shelf_id = match Uuid::parse_str(&payload.shelf_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!(ErrorResponse { error: "Invalid shelf ID.".to_string() }))),
    };

    let shelf = match crate::schema::shelves::dsl::shelves
        .filter(crate::schema::shelves::dsl::id.eq(shelf_id))
        .first::<Shelf>(connection)
    {
        Ok(s) => s,
        Err(_) => return (StatusCode::NOT_FOUND, Json(json!(ErrorResponse { error: "Shelf not found.".to_string() }))),
    };

    if shelf.user != auth.0 {
        return (StatusCode::FORBIDDEN, Json(json!(ErrorResponse { error: "Access denied.".to_string() })));
    }

    let results = match books
        .filter(crate::schema::books::dsl::shelf.eq(shelf_id))
        .load::<Book>(connection)
    {
        Ok(r) => r,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ErrorResponse { error: format!("Error loading books: {}", e) })),
        ),
    };

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

/// Request type for adding a book to a shelf.
#[derive(Debug, Deserialize)]
pub struct AddBookToShelfRequest {
    pub shelf_id: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub isbn13: Option<String>,
    pub isbn10: Option<String>,
    pub google_books_id: Option<String>,
}

/// Adds a book to a shelf.
pub(crate) async fn add_book_to_shelf(
    auth: AuthUser,
    Json(payload): Json<AddBookToShelfRequest>,
) -> impl IntoResponse {
    let connection = &mut connect();

    let shelf_id = match Uuid::parse_str(&payload.shelf_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!(ErrorResponse { error: "Invalid shelf ID.".to_string() }))),
    };

    let shelf: Shelf = match crate::schema::shelves::dsl::shelves
        .filter(crate::schema::shelves::dsl::id.eq(shelf_id))
        .first(connection)
    {
        Ok(s) => s,
        Err(_) => return (StatusCode::NOT_FOUND, Json(json!(ErrorResponse { error: "Shelf not found.".to_string() }))),
    };

    if shelf.user != auth.0 {
        return (StatusCode::FORBIDDEN, Json(json!(ErrorResponse { error: "Access denied.".to_string() })));
    }

    let new_book = Book {
        id: Uuid::new_v4(),
        user: auth.0,
        shelf: shelf_id,
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
        Ok(_) => (StatusCode::CREATED, Json(json!({ "message": "Book added to shelf successfully." }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(ErrorResponse { error: format!("Error while adding the book to the shelf: {}", e) }))),
    }
}

/// Request type for removing a book from a shelf.
#[derive(Debug, Deserialize)]
pub struct RemoveBookFromShelfRequest {
    pub book_id: String,
}

/// Removes a book from a shelf.
pub(crate) async fn remove_book_from_shelf(
    auth: AuthUser,
    Json(payload): Json<RemoveBookFromShelfRequest>,
) -> impl IntoResponse {
    let connection = &mut connect();

    let book_id = match Uuid::parse_str(&payload.book_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!(ErrorResponse { error: "Invalid book ID.".to_string() }))),
    };

    let book: Book = match crate::schema::books::dsl::books
        .filter(crate::schema::books::dsl::id.eq(book_id))
        .first(connection)
    {
        Ok(b) => b,
        Err(_) => return (StatusCode::NOT_FOUND, Json(json!(ErrorResponse { error: "Book not found.".to_string() }))),
    };

    if book.user != auth.0 {
        return (StatusCode::FORBIDDEN, Json(json!(ErrorResponse { error: "Access denied.".to_string() })));
    }

    match diesel::delete(
        crate::schema::books::dsl::books.filter(crate::schema::books::dsl::id.eq(book_id)),
    ).execute(connection) {
        Ok(_) => (StatusCode::OK, Json(json!({ "message": "Book removed from shelf successfully." }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(ErrorResponse { error: format!("Error while removing the book from the shelf: {}", e) }))),
    }
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request, routing::post, Router};
    use tower::ServiceExt;
    use super::*;

    #[tokio::test]
    async fn test_list_shelves_requires_auth() {
        let app = Router::new().route("/api/shelves", post(list_shelves));
        let response = app
            .oneshot(Request::builder().method("POST").uri("/api/shelves").body(Body::empty()).unwrap())
            .await.unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_create_shelf_requires_auth() {
        let app = Router::new().route("/api/shelves/create", post(create_shelf));
        let response = app
            .oneshot(Request::builder().method("POST").uri("/api/shelves/create").body(Body::empty()).unwrap())
            .await.unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_remove_shelf_requires_auth() {
        let app = Router::new().route("/api/shelves/remove", post(remove_shelf));
        let response = app
            .oneshot(Request::builder().method("POST").uri("/api/shelves/remove").body(Body::empty()).unwrap())
            .await.unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_add_book_requires_auth() {
        let app = Router::new().route("/api/shelves/add-book", post(add_book_to_shelf));
        let response = app
            .oneshot(Request::builder().method("POST").uri("/api/shelves/add-book").body(Body::empty()).unwrap())
            .await.unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_list_shelf_books_requires_auth() {
        let app = Router::new().route("/api/shelves/books", post(list_shelf_books));
        let response = app
            .oneshot(Request::builder().method("POST").uri("/api/shelves/books").body(Body::empty()).unwrap())
            .await.unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_remove_book_requires_auth() {
        let app = Router::new().route("/api/shelves/remove-book", post(remove_book_from_shelf));
        let response = app
            .oneshot(Request::builder().method("POST").uri("/api/shelves/remove-book").body(Body::empty()).unwrap())
            .await.unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }
}
