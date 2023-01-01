use diesel::{
    prelude::*,
    sql_types::{Timestamp, Uuid},
};

#[derive(Queryable)]
pub struct Paste {
    pub id: Uuid,
    pub file: String,
    pub created_at: Timestamp,
    pub expires_at: Timestamp,
}
