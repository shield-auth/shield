use crate::client;
use sea_orm::{prelude::Uuid, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

impl client::Entity {
    pub async fn find_active_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<client::Model>, DbErr> {
        Self::find_by_id(id).filter(client::Column::LockedAt.is_null()).one(db).await
    }
}
