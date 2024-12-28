use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Json
};
use serde_json::json;
use crate::utils;

pub async fn verify_token(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    let Some(auth_value) = auth_header else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(
                json!({
                    "error": "Missing or invalid Authorization header"
                })
            )
        ));
    };

    if !auth_value.starts_with("Bearer ") {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(
                json!({
                    "error": "Expected Bearer token"
                })
            )
        ));
    }

    let token = auth_value.trim_start_matches("Bearer ");

    let user_id = match utils::token::decode_token(token) {
        Ok(t) => t,
        Err(_e) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(
                    json!({
                        "error": "Invalid token"
                    })
                )
            ));
        }
    };

    req.extensions_mut().insert(user_id);

    let response = next.run(req).await;
    Ok(response)
}