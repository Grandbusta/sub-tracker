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
    Json(body): Json<models::user::CreateUserReq>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {

    let hashed_password = utils::password::hash_password(&body.password);
    let user = models::user::CreateUserReq {
        email: body.email,
        password: hashed_password
    };

    let created_user=db::user::save_user(&user, &db_pool).await;

    match created_user {
        Ok(user) => {
            return Ok((
                StatusCode::CREATED,
                Json(
                    json!({
                        "id": user.id,
                        "email": user.email,
                        "created_at": user.created_at
                    })
                )
            ))
        },
        Err(sqlx::Error::Database(err)) => {
            if err.is_unique_violation() {
                return Ok((
                    StatusCode::BAD_REQUEST,
                    Json(
                        json!({
                            "message": "User already exists"
                        })
                    )
                ))
            }else{
                return Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        json!({
                            "message": "Failed to create user"
                        })
                    )
                ))
            }
        },
        Err(e) => {
            return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({
                        "message": "Failed to create user"
                    })
                )
            ))
        }
    }
}

pub async fn login_user(
    State(db_pool): State<PgPool>,
    Json(body): Json<models::user::CreateUserReq>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {

    let existing_user = match db::user::get_user_by_email(
        &body.email, &db_pool
    ).await {
        Ok(user) => user,
        Err(e) => return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                json!({
                    "message": "Failed to get user"
                })
            )
        ))
    };
    
    let user = existing_user.ok_or((
        StatusCode::BAD_REQUEST,
        Json(
            json!({
                "message": "Wrong credentials"
            })
        )
    ))?;
    

    let password_matches = utils::password::compare_password(
        &body.password, &user.password
    );

    if !password_matches {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(
                json!({
                    "message": "Wrong credentials"
                })
            )
        ))
    }

let token = match utils::token::create_token(
    &user.id.unwrap().to_string()
) {
    Ok(t) => t,
    Err(_e) => {
        // Return an HTTP 500 (or another code) plus JSON message
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Failed to create token"
            }))
        ));
    }
};
    
    Ok((StatusCode::OK, Json(json!({
        "message": "Login successful",
        "token": token
    }))))
}