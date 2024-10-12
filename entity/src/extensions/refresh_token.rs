use crate::refresh_token;
use sea_orm::{prelude::Uuid, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

impl refresh_token::Entity {
    pub async fn find_active_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<refresh_token::Model>, DbErr> {
        Self::find_by_id(id).filter(refresh_token::Column::LockedAt.is_null()).one(db).await
    }
}
