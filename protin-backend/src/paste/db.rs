use std::cmp;
use std::time::{Duration, SystemTime};

use anyhow::Context;
use diesel::{dsl, prelude::*};

use crate::models::{NewPaste, Paste};

pub fn create_new_paste(
    conn: &mut PgConnection,
    id: String,
    expires_at: Option<u64>,
) -> anyhow::Result<Paste> {
    use crate::schema::pastes;

    let expires_at = expires_at
        .map(|raw_timestamp| SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(raw_timestamp)))
        .flatten();
    let new_paste = NewPaste { id, expires_at };

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

    pastes.sort_by_key(|a| cmp::Reverse(a.expires_at));

    let latest_paste = pastes.first();
    if let Some(paste) = latest_paste {
        Ok(Some(paste.clone()))
    } else {
        Ok(None)
    }
}

pub fn get_expired_paste_ids(conn: &mut PgConnection) -> anyhow::Result<Vec<String>> {
    use crate::schema::pastes;

    pastes::table
        .filter(pastes::expires_at.lt(dsl::now))
        .filter(pastes::deleted.eq(false))
        .select(pastes::id)
        .load::<String>(conn)
        .context("Can't get expired records from pastes table")
}

pub fn set_deleted_for_ids(
    conn: &mut PgConnection,
    deleted_ids: Vec<String>,
) -> anyhow::Result<usize> {
    use crate::schema::pastes;

    diesel::update(pastes::table)
        .filter(pastes::id.eq_any(deleted_ids))
        .set(pastes::deleted.eq(true))
        .execute(conn)
        .context("Can't update the deletes column on pastes table")
}
