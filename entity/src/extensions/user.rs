use crate::models::user;
use sea_orm::{prelude::Uuid, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

impl user::Model {
    pub fn verify_password(&self, password: &str) -> bool {
        match self.password_hash {
            Some(ref hash) => bcrypt::verify(password, hash).unwrap_or(false),
            None => false,
        }
    }
}

impl user::Entity {
    pub async fn find_active_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        Self::find_by_id(id).filter(user::Column::LockedAt.is_null()).one(db).await
    }
}
