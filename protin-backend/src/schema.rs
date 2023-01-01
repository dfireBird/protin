// @generated automatically by Diesel CLI.

diesel::table! {
    pastes (id) {
        id -> Uuid,
        file -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}
