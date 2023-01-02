use std::time::SystemTime;

use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Paste {
    pub id: Uuid,
    pub file: String,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}
