use anyhow::Context;
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{NewPaste, Paste};

pub fn create_new_paste(
    conn: &mut PgConnection,
    id: String,
    file_path: Uuid,
) -> anyhow::Result<Paste> {
    use crate::schema::pastes;

    let new_paste = NewPaste { id, file_path };

    diesel::insert_into(pastes::table)
        .values(new_paste)
        .get_result(conn)
        .context("Can't insert a record into pastes table")
}
