use crate::api_user;
use sea_orm::{prelude::Uuid, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

impl api_user::Entity {
    pub async fn find_active_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<api_user::Model>, DbErr> {
        Self::find_by_id(id).filter(api_user::Column::LockedAt.is_null()).one(db).await
    }
}
