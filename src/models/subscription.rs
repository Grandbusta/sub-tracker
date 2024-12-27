use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Type)]
#[sqlx(type_name = "frequency")]
#[sqlx(rename_all = "lowercase")]
pub enum Frequency {
    Weekly,
    Monthly,
    Yearly,
}

impl Frequency {
    pub fn to_string(&self) -> &str {
        match self {
            Frequency::Weekly => "weekly",
            Frequency::Monthly => "monthly",
            Frequency::Yearly => "yearly",
        }
    }
}

#[derive(Debug,Deserialize, Serialize)]
pub struct Subscription {
    pub id: Option<uuid::Uuid>,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub website_url: String,
    pub price: i32,
    pub frequency: Frequency,
    pub category: String,
    pub date_started: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>
}

#[derive(Debug,Deserialize, Serialize)]
pub struct CreateSubscriptionReq {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub website_url: String,
    pub price: i32,
    pub frequency: Frequency,
    pub category: String,
    pub date_started: NaiveDateTime
}