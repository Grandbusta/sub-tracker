use sqlx::{Pool, Postgres};
use crate::models;
use crate::models::subscription::Frequency;

pub async fn save_subscription(
    subscription: &models::subscription::CreateSubscriptionReq,
    db_pool: &Pool<Postgres>
) -> Result<models::subscription::Subscription, sqlx::Error> {
    let new_subscription = sqlx::query_as!(
        models::subscription::Subscription,
        r#"INSERT INTO subscriptions
            (user_id, name, website_url, price, category, frequency, date_started) 
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING
        id, user_id, name,
        website_url, price, category,
        frequency as "frequency: Frequency",
        date_started, created_at"#,
        subscription.user_id,
        subscription.name,
        subscription.website_url,
        subscription.price,
        subscription.category,
        subscription.frequency.to_string(),
        subscription.date_started
    ).fetch_one(db_pool).await?;

    Ok(new_subscription)
}