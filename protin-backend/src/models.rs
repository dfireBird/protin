use std::time::SystemTime;

use diesel::prelude::*;
use serde::Serialize;

use crate::schema::pastes;

#[derive(Clone, Debug, Queryable, Serialize)]
pub struct Paste {
    pub id: String,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
    pub deleted: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = pastes)]
pub struct NewPaste {
    pub id: String,
    pub expires_at: Option<SystemTime>,
}
