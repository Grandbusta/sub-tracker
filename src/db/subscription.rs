use sqlx::{Pool, Postgres};
use crate::models;
use crate::models::subscription::Frequency;

pub async fn save_subscription(
    subscription: &models::subscription::CreateSubscriptionData,
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

pub async fn get_subscriptions_by_user_id(
    user_id: &uuid::Uuid,
    db_pool: &Pool<Postgres>
) -> Result<Vec<models::subscription::SubscriptionDBResponse>, sqlx::Error> {
    let subscriptions = sqlx::query_as!(
        models::subscription::SubscriptionDBResponse,
        r#"SELECT
            id, name,
            website_url, price, category,
            frequency as "frequency: Frequency",
            date_started, created_at
        FROM subscriptions WHERE user_id = $1
        ORDER BY created_at DESC"#,
        user_id
    ).fetch_all(db_pool).await?;

    Ok(subscriptions)
}


pub async fn get_subscription_by_subscription_id(
    subscription_id: &uuid::Uuid,
    db_pool: &Pool<Postgres>
) -> Result<models::subscription::Subscription, sqlx::Error> {
    let subscription = sqlx::query_as!(
        models::subscription::Subscription,
        r#"SELECT
            id,user_id, name,
            website_url, price, category,
            frequency as "frequency: Frequency",
            date_started, created_at
        FROM subscriptions WHERE id = $1"#,
        subscription_id
    ).fetch_one(db_pool).await?;

    Ok(subscription)
}

pub async fn update_subscription(
    subscription_id: &uuid::Uuid,
    subscription: &models::subscription::UpdateSubscriptionData,
    db_pool: &Pool<Postgres>
) -> Result<models::subscription::Subscription, sqlx::Error> {
    let new_subscription = sqlx::query_as!(
        models::subscription::Subscription,
        r#"UPDATE subscriptions
            SET
                name = COALESCE($2, name),
                website_url = COALESCE($3, website_url),
                price = COALESCE($4, price),
                category = COALESCE($5, category),
                frequency = COALESCE($6, frequency),
                date_started = COALESCE($7, date_started)
        WHERE id = $1
        RETURNING
            id, user_id, name,
            website_url, price, category,
            frequency as "frequency: Frequency",
            date_started, created_at"#,
        subscription_id,
        subscription.name,
        subscription.website_url,
        subscription.price,
        subscription.category,
        subscription.frequency,
        subscription.date_started
    ).fetch_one(db_pool).await?;

    Ok(new_subscription)
}


pub async fn delete_subscription_by_subscription_id(
    subscription_id: &uuid::Uuid,
    db_pool: &Pool<Postgres>
) -> Result<bool, sqlx::Error> {
    sqlx::query_as!(
        models::subscription::Subscription,
        r#"DELETE FROM subscriptions WHERE id = $1"#,
        subscription_id
    ).execute(db_pool).await?;

    Ok(true)
}
