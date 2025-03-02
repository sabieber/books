// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "reading_mode"))]
    pub struct ReadingMode;
}

diesel::table! {
    books (id) {
        id -> Uuid,
        user -> Uuid,
        shelf -> Uuid,
        title -> Nullable<Text>,
        author -> Nullable<Text>,
        isbn13 -> Nullable<Text>,
        isbn10 -> Nullable<Text>,
        google_books_id -> Nullable<Text>,
        added_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ReadingMode;

    reading_entries (id) {
        id -> Uuid,
        reading -> Uuid,
        book -> Uuid,
        user -> Uuid,
        progress -> Int4,
        mode -> ReadingMode,
        read_at -> Date,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ReadingMode;

    readings (id) {
        id -> Uuid,
        book -> Uuid,
        user -> Uuid,
        total_pages -> Int4,
        progress -> Int4,
        mode -> ReadingMode,
        started_at -> Date,
        finished_at -> Nullable<Date>,
        cancelled_at -> Nullable<Date>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    shelves (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        user -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 32]
        name -> Varchar,
        password -> Text,
        elevated -> Bool,
    }
}

diesel::joinable!(books -> shelves (shelf));
diesel::joinable!(books -> users (user));
diesel::joinable!(reading_entries -> books (book));
diesel::joinable!(reading_entries -> readings (reading));
diesel::joinable!(reading_entries -> users (user));
diesel::joinable!(readings -> books (book));
diesel::joinable!(readings -> users (user));
diesel::joinable!(shelves -> users (user));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    reading_entries,
    readings,
    shelves,
    users,
);
