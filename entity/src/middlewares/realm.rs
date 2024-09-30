use crate::models::realm::ActiveModel;
use async_trait::async_trait;
use sea_orm::{entity::prelude::*, ActiveValue};
use slug::slugify;

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Will be triggered before insert / update
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        println!("🚀 Before save {:#?}", self);
        if let ActiveValue::Set(ref name) = self.name {
            let slug = slugify(name);
            self.slug = ActiveValue::Set(slug);
        }

        Ok(self)
    }
}
