use crate::{models::resource_group, utils::check_locked_at_constraint};
use async_trait::async_trait;
use sea_orm::{entity::prelude::*, ActiveValue};

#[async_trait]
impl ActiveModelBehavior for resource_group::ActiveModel {
    /// Will be triggered before insert / update
    async fn before_save<C>(mut self, db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if let ActiveValue::Set(ref locked_at) = self.locked_at {
            check_locked_at_constraint(locked_at)?
        }

        // If the new row is being set as default
        if let ActiveValue::Set(is_default) = self.is_default {
            if is_default {
                // Set all other resource groups for the same user and client to non-default
                resource_group::Entity::update_many()
                    .col_expr(resource_group::Column::IsDefault, Expr::value(false))
                    .filter(resource_group::Column::UserId.eq(self.user_id.clone().unwrap()))
                    .filter(resource_group::Column::ClientId.eq(self.client_id.clone().unwrap()))
                    .filter(resource_group::Column::Id.ne(self.id.clone().unwrap()))
                    .exec(db)
                    .await?;
            } else {
                // Check if this was the only default group
                let default_exists = resource_group::Entity::find()
                    .filter(resource_group::Column::UserId.eq(self.user_id.clone().unwrap()))
                    .filter(resource_group::Column::ClientId.eq(self.client_id.clone().unwrap()))
                    .filter(resource_group::Column::IsDefault.eq(true))
                    .filter(resource_group::Column::Id.ne(self.id.clone().unwrap()))
                    .one(db)
                    .await?
                    .is_some();

                // If no other default exists, force this group to be default
                if !default_exists {
                    self.is_default = ActiveValue::Set(true);
                }
            }
        }

        Ok(self)
    }
}
