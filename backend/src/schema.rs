// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 32]
        name -> Varchar,
        password -> Text,
        salt -> Text,
        elevated -> Bool,
    }
}
