use axum::{
    extract::State,
    http::StatusCode,
    Json
};
use serde_json::json;
use crate::models;
use crate::db;
use crate::AppState;


pub async fn create_subscription(
    State(app_state): State<AppState>,
    Json(body): Json<models::subscription::CreateSubscriptionReq>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {

    let created_subscription=db::subscription::save_subscription(&body, &app_state.db_pool).await;

    match created_subscription {
        Ok(subscription) => {
            return Ok((
                StatusCode::CREATED,
                Json(
                    json!({
                        "message": "Subscription created successfully",
                        "data":{
                            "id": subscription.id,
                            "user_id": subscription.user_id,
                            "name": subscription.name,
                            "website_url": subscription.website_url,
                            "price": subscription.price,
                            "frequency": subscription.frequency,
                            "category": subscription.category,
                            "date_started": subscription.date_started,
                            "created_at": subscription.created_at
                        }
                    })
                )
            ))
        },
        Err(err) => {
            println!("{:?}", err);
            return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({
                        "message": "Failed to create subscription"
                    })
                )
            ))
        }
    }
}