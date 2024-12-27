use axum::{
    extract::{Path,State},
    routing::{get, post},
    Router
};
use sqlx::{pool::PoolOptions, Pool, Postgres};
use tokio::net::TcpListener;
use dotenvy::dotenv;
use std::env;

mod models;
mod controllers;
mod utils;
mod db;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv().expect("Failed to load .env file");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool: Pool<Postgres> = PoolOptions::new()
        .max_connections(10).connect(&db_url)
        .await.expect("Failed to connect to database");

    let shared_state = AppState {
        db_pool,
    };

    sqlx::migrate!("src/db/migrations").run(&shared_state.db_pool).await.expect("Failed to run migrations");

    let app = Router::new()
        .nest("/user", user_routes())
        .nest("/subscription", subscription_routes())
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.expect("Failed to serve");
}


fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(controllers::user::create_user))
        .route("/login", post(controllers::user::login_user))
}

fn subscription_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(controllers::subscription::create_subscription))
}