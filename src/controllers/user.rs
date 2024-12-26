use axum::{
    extract::{Path,State},
    http::StatusCode,
    routing::{get, post},
    Json
};
use serde_json::json;
use sqlx::PgPool;
use crate::models;
use crate::utils;
use crate::db;

pub async fn create_user(
    State(db_pool): State<PgPool>,
    Json(user): Json<models::user::CreateUserReq>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {

    let existing_user = db::user::get_user_by_email(
        &user.email, &db_pool
    ).await;

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

    let hashed_password = utils::password::hash_password(&user.password);
    let user = models::user::CreateUserReq {
        email: user.email,
        password: hashed_password
    };

    let created_user=db::user::save_user(&user, &db_pool).await.map_err(|e|(
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(
            json!({
                "message": "Failed to create user"
            })
        )
    ) )?;

    Ok((StatusCode::CREATED, 
        Json(json!({
        "id": created_user.id,
        "email": created_user.email,
        "created_at": created_user.created_at
    }))
))
}

pub async fn login_user(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::OK, Json(json!({
        "message": "Login successful"
    }))))
}