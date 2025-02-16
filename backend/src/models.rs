use diesel::internal::derives::multiconnection::chrono;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub elevated: bool,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::shelves)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct Shelf {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub user: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Shelf))]
pub struct Book {
    pub id: Uuid,
    pub user: Uuid,
    pub shelf: Uuid,
    pub title: Option<String>,
    pub author: Option<String>,
    pub isbn13: Option<String>,
    pub isbn10: Option<String>,
    pub google_books_id: Option<String>,
    pub added_at: chrono::NaiveDateTime,
}
