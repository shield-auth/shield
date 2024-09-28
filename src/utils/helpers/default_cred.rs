use std::str::FromStr;

use sea_orm::prelude::Uuid;
use serde::Deserialize;

use crate::packages::errors::Error;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DefaultCred {
    pub realm_id: Uuid,
    pub client_id: Uuid,
}

impl DefaultCred {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let mut realm_id = String::new();
        let mut client_id = String::new();

        for line in s.lines() {
            if line.contains("realm_id:") {
                realm_id = line
                    .split(':')
                    .nth(1)
                    .map(|s| s.trim().trim_end_matches(',').to_string())
                    .unwrap_or_default();
            }
            if line.contains("client_id:") {
                client_id = line
                    .split(':')
                    .nth(1)
                    .map(|s| s.trim().trim_end_matches(',').to_string())
                    .unwrap_or_default();
            }
        }

        if realm_id.is_empty() || client_id.is_empty() {
            return Err(Error::invalid_input("Failed to parse DefaultCred"));
        }

        let realm_id = Uuid::from_str(&realm_id).map_err(|_| Error::invalid_input("Failed to parse DefaultCred"))?;
        let client_id = Uuid::from_str(&client_id).map_err(|_| Error::invalid_input("Failed to parse DefaultCred"))?;
        Ok(DefaultCred { realm_id, client_id })
    }
}
