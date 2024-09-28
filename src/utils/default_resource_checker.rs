use std::fs::read_to_string;

use sea_orm::prelude::Uuid;

use crate::packages::errors::Error;

use super::helpers::default_cred::DefaultCred;

pub fn is_default_realm(realm_id: Uuid) -> bool {
    let default_cred_str = read_to_string("./logs/default_cred.txt").map_err(Error::from);
    let default_cred_str = default_cred_str.unwrap();
    let default_cred = DefaultCred::from_str(&default_cred_str);
    let default_cred = default_cred.unwrap();

    realm_id == default_cred.realm_id
}
