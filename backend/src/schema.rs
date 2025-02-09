// @generated automatically by Diesel CLI.

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

diesel::joinable!(shelves -> users (user));

diesel::allow_tables_to_appear_in_same_query!(
    shelves,
    users,
);
