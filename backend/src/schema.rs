// @generated automatically by Diesel CLI.

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
diesel::joinable!(shelves -> users (user));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    shelves,
    users,
);
