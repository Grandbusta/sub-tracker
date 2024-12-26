use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: Option<uuid::Uuid>,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>
}


#[derive(Deserialize, Serialize)]
pub struct CreateUserReq {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserRes {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime
}