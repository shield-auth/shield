use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schemas::db::realm;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = realm)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Realm {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub locked_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = realm)]
pub struct NewRealm<'a> {
    pub name: &'a str,
}
