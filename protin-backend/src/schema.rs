// @generated automatically by Diesel CLI.

diesel::table! {
    pastes (id) {
        file_path -> Uuid,
        id -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}
