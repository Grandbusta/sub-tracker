use sqlx::{Pool, Postgres};
use crate::models;

pub async fn get_user_by_email(
    email: &str,
    db_pool: &Pool<Postgres>
) -> Result<Option<models::user::User>, sqlx::Error> {
    let user = sqlx::query_as!(
        models::user::User,
        "SELECT * FROM users WHERE email = $1",
        email
    ).fetch_optional(db_pool).await?;

    Ok(user)
}

pub async fn save_user(
    user: &models::user::CreateUserReq,
    db_pool: &Pool<Postgres>
) -> Result<models::user::User, sqlx::Error> {
    let new_user = sqlx::query_as!(
        models::user::User,
        "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *",
        user.email,
        user.password
    ).fetch_one(db_pool).await?;

    Ok(new_user)
}