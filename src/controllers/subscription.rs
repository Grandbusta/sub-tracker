use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    Json
};
use serde_json::json;
use crate::models;
use crate::db;
use crate::AppState;


pub async fn create_subscription(
    Extension(user_id): Extension<String>,
    State(app_state): State<AppState>,
    Json(body): Json<models::subscription::CreateSubscriptionReq>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {

    let create_subscription_data = models::subscription::CreateSubscriptionData {
        user_id: user_id.parse().unwrap(),
        name: body.name,
        website_url: body.website_url,
        price: body.price,
        frequency: body.frequency,
        category: body.category,
        date_started: body.date_started
    };
    let created_subscription=db::subscription::save_subscription(
        &create_subscription_data,
        &app_state.db_pool
    ).await;

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

pub async fn get_user_subscriptions(
    Path(user_id): Path<String>,
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let subscriptions = db::subscription::get_subscriptions_by_user_id(
        &user_id.parse().unwrap(),
        &app_state.db_pool
    ).await;

    match subscriptions {
        Ok(subscriptions) => {
            return Ok((
                StatusCode::OK,
                Json(
                    json!({
                        "message": "Subscriptions retrieved successfully",
                        "data": subscriptions
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
                        "message": "Failed to get subscriptions"
                    })
                )
            ))
        }
    }
}


pub async fn get_subscription_by_id(
    Path(subscription_id): Path<String>,
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let subscription = db::subscription::get_subscription_by_subscription_id(
        &subscription_id.parse().unwrap(),
        &app_state.db_pool
    ).await;
    
    match subscription {
        Ok(subscription) => {
            return Ok((
                StatusCode::OK,
                Json(
                    json!({
                        "message": "Subscription retrieved successfully",
                        "data": subscription
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
                        "message": "Failed to get subscription"
                    })
                )
            ))
        }
    }
}

pub async fn update_subscription(
    Path(subscription_id): Path<String>,
    State(app_state): State<AppState>,
    Json(body): Json<models::subscription::UpdateSubscriptionData>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let subscription = db::subscription::update_subscription(
        &subscription_id.parse().unwrap(),
        &body,
        &app_state.db_pool
    ).await;
    
    match subscription {
        Ok(subscription) => {
            return Ok((
                StatusCode::OK,
                Json(
                    json!({
                        "message": "Subscription updated successfully",
                        "data": subscription
                    })
                )
            ))
        },
        Err(sqlx::Error::RowNotFound) => {
            return Ok((
                StatusCode::NOT_FOUND,
                Json(
                    json!({
                        "message": "Subscription not found"
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
                        "message": "Failed to update subscription"
                    })
                )
            ))
        }
    }
}


pub async fn delete_subscription(
    Path(subscription_id): Path<String>,
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let subscription = db::subscription::delete_subscription_by_subscription_id(
        &subscription_id.parse().unwrap(),
        &app_state.db_pool
    ).await;
    
    match subscription {
        Ok(_subscription) => {
            return Ok((
                StatusCode::OK,
                Json(
                    json!({
                        "message": "Subscription deleted successfully"
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
                        "message": "Failed to delete subscription"
                    })
                )
            ))
        }
    }
}