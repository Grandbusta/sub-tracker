use axum::{
    extract::{Path,State},
    http::StatusCode,
    routing::{get, post},
    Json
};
use serde_json::json;
use sqlx::PgPool;
use crate::models;

pub async fn create_user(
    State(db_pool): State<PgPool>,
    Json(user): Json<models::CreateUserReq>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {

    let existing_user = sqlx::query!(
        "SELECT * FROM users WHERE email = $1",
        user.email
    ).fetch_one(&db_pool).await.map_err(|e|(
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({
            "message": "Failed to create user"
        }).to_string()
    ) );

    if existing_user.is_ok() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(
                json!({
                    "message": "User already exists"
                })
            )
        ))
    }

    let row = sqlx::query_as!(
        models::User,
        "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *",
        user.email,
        user.password
    ).fetch_one(&db_pool).await.map_err(|e|(
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(
            json!({
                "message": "Failed to create user"
            })
        )
    ) )?;

    Ok((StatusCode::CREATED, 
        Json(json!({
        "id": row.id,
        "email": row.email,
        "password": row.password,
        "created_at": row.created_at
    }))
))
}