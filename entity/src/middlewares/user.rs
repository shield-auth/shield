use crate::{models::user, utils::check_locked_at_constraint};
use async_trait::async_trait;
use regex::Regex;
use sea_orm::{entity::prelude::*, sqlx::types::chrono::Utc, ActiveValue};

#[async_trait]
impl ActiveModelBehavior for user::ActiveModel {
    /// Will be triggered before insert / update
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if let ActiveValue::Set(ref locked_at) = self.locked_at {
            check_locked_at_constraint(locked_at)?
        }

        // Check email format constraint
        if let ActiveValue::Set(ref email) = self.email {
            let email_regex = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$")
                .map_err(|e| DbErr::Custom(format!("Failed to compile email regex: {}", e)))?;

            if !email_regex.is_match(email) {
                return Err(DbErr::Custom(format!("Invalid email format: {}", email)));
            }
        }

        // Check email_verified_at constraint
        if let (ActiveValue::Set(Some(email_verified_at)), ActiveValue::Set(created_at)) = (&self.email_verified_at, &self.created_at) {
            if email_verified_at < created_at || email_verified_at > &Utc::now() {
                return Err(DbErr::Custom("Email verified date must be between created date and now".to_owned()));
            }
        }

        // Check phone format constraint
        if let ActiveValue::Set(Some(ref phone)) = self.phone {
            let phone_regex = Regex::new(r"^\+?[0-9]{10,14}$").map_err(|e| DbErr::Custom(format!("Failed to compile phone regex: {}", e)))?;

            if !phone_regex.is_match(phone) {
                return Err(DbErr::Custom(format!("Invalid phone number format: {}", phone)));
            }
        }

        Ok(self)
    }
}
