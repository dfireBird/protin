use anyhow::Context;
use diesel::dsl;
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

pub fn get_paste(conn: &mut PgConnection, rid: String) -> anyhow::Result<Option<Paste>> {
    use crate::schema::pastes::dsl as pastes_dsl;

    let mut pastes = pastes_dsl::pastes
        .filter(pastes_dsl::id.eq(rid.clone()))
        .filter(pastes_dsl::expires_at.gt(dsl::now))
        .load::<Paste>(conn)
        .context("Can't get records from pastes table")?;

    pastes.sort_by(|a, b| b.expires_at.cmp(&a.expires_at));

    let latest_paste = pastes.get(0);
    if let Some(paste) = latest_paste {
        Ok(Some(paste.clone()))
    } else {
        Ok(None)
    }
}
