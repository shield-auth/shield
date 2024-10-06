use sea_orm::{
    sqlx::types::chrono::{FixedOffset, Utc},
    DbErr,
};

pub fn check_locked_at_constraint(locked_at: &Option<sea_orm::sqlx::types::chrono::DateTime<FixedOffset>>) -> Result<(), DbErr> {
    if locked_at.is_some() {
        // NOTE: If required, we can enable the feature to lock the client in the future.
        if locked_at.unwrap() > Utc::now() {
            return Err(DbErr::Custom("Cannot lock the client".to_owned()));
        }
    }
    Ok(())
}
