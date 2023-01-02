use std::time::SystemTime;

use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Paste {
    pub file_path: Uuid,
    pub id: String,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}
