use chrono::{DateTime, Duration, FixedOffset, Utc};
use entity::sea_orm_active_enums::{ApiUserAccess, ApiUserRole};
use serde::Deserialize;

#[derive(Deserialize)]
pub enum TokenExpires {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "7d")]
    OneWeek,
    #[serde(rename = "1m")]
    OneMonth,
    #[serde(rename = "3m")]
    ThreeMonths,
    #[serde(rename = "1year")]
    OneYear,
}

impl TokenExpires {
    pub fn to_datetime(&self) -> DateTime<FixedOffset> {
        let utc_time = match self {
            TokenExpires::OneHour => Utc::now() + Duration::hours(1),
            TokenExpires::OneDay => Utc::now() + Duration::days(1),
            TokenExpires::OneWeek => Utc::now() + Duration::weeks(1),
            TokenExpires::OneMonth => Utc::now() + Duration::days(30),
            TokenExpires::ThreeMonths => Utc::now() + Duration::days(90),
            TokenExpires::OneYear => Utc::now() + Duration::weeks(52),
            _ => Utc::now() + Duration::weeks(52 * 99),
        };

        utc_time.into()
    }
}

#[derive(Deserialize)]
pub struct CreateApiUserRequest {
    pub name: String,
    pub description: Option<String>,
    pub role: ApiUserRole,
    pub access: ApiUserAccess,
    pub expires: Option<TokenExpires>,
}
