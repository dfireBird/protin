// @generated automatically by Diesel CLI.

diesel::table! {
    pastes (id) {
        id -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        deleted -> Bool,
    }
}
