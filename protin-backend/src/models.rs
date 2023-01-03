use std::time::SystemTime;

use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::pastes;

#[derive(Queryable)]
pub struct Paste {
    pub file_path: Uuid,
    pub id: String,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = pastes)]
pub struct NewPaste {
    pub id: String,
    pub file_path: Uuid,
}
