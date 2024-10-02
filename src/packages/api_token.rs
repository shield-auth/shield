use sea_orm::{
    prelude::{DateTimeWithTimeZone, Uuid},
    DatabaseConnection, EntityTrait,
};
use serde::{Deserialize, Serialize};

use entity::{
    api_user,
    sea_orm_active_enums::{ApiUserAccess, ApiUserRole},
};

use super::errors::{AuthenticateError, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTokenUser {
    pub id: Uuid,
    pub realm_id: Uuid,
    pub client_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub role: ApiUserRole,
    pub access: ApiUserAccess,
    pub expires: DateTimeWithTimeZone,
}

impl ApiTokenUser {
    fn from(api_user: api_user::Model) -> Self {
        Self {
            id: api_user.id,
            realm_id: api_user.realm_id,
            client_id: api_user.client_id,
            name: api_user.name,
            description: api_user.description,
            role: api_user.role,
            access: api_user.access,
            expires: api_user.expires,
        }
    }

    pub async fn validate_cred(db: &DatabaseConnection, api_key: &str) -> Result<ApiTokenUser, Error> {
        let parts = api_key.split('.').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(Error::Authenticate(AuthenticateError::InvalidApiCredentials));
        }
        let id = parts[0]
            .parse::<Uuid>()
            .map_err(|_| Error::Authenticate(AuthenticateError::InvalidApiCredentials))?;
        let secret = parts[1];

        println!("🚀 Api Key inside validate_cred: {}", api_key);

        let api_user = api_user::Entity::find_by_id(id).one(db).await?;
        if api_user.is_none() {
            return Err(Error::Authenticate(AuthenticateError::InvalidApiCredentials));
        }

        let api_user = api_user.unwrap();

        if api_user.expires.timestamp() <= chrono::Local::now().timestamp() {
            return Err(Error::Authenticate(AuthenticateError::InvalidApiCredentials));
        }

        if api_user.secret != secret {
            return Err(Error::Authenticate(AuthenticateError::InvalidApiCredentials));
        }

        Ok(Self::from(api_user))
    }
}
