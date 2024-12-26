use axum::{
    extract::{Path,State},
    http::StatusCode,
    routing::{get, post},
    Router
};
use sqlx::{pool::PoolOptions, Pool, Postgres};
use tokio::net::TcpListener;
use dotenvy::dotenv;
use std::env;

mod models;
mod controllers;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv().expect("Failed to load .env file");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool: Pool<Postgres> = PoolOptions::new()
        .max_connections(10).connect(&db_url)
        .await.expect("Failed to connect to database");

    sqlx::migrate!("src/migrations").run(&db_pool).await.expect("Failed to run migrations");

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/user", post(controllers::create_user))
        .with_state(db_pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.expect("Failed to serve");
}


async fn hello_world() -> &'static str {
    return "Hello World!"
}
