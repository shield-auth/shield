use crate::{models::resource, utils::check_locked_at_constraint};
use async_trait::async_trait;
use sea_orm::{entity::prelude::*, ActiveValue};

#[async_trait]
impl ActiveModelBehavior for resource::ActiveModel {
    /// Will be triggered before insert / update
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if let ActiveValue::Set(ref locked_at) = self.locked_at {
            check_locked_at_constraint(locked_at)?
        }

        Ok(self)
    }
}
