use std::env;
use axum::Router;
use axum::routing::get;
use sqlx::mysql::MySqlPoolOptions;

use db_web::routes::router::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let base_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&base_url)
        .await
        .expect("failed to connect to database");

    let app = Router::new()
        .nest("/user", user_routes())
        .nest("/client", client_routes())
        .nest("/repository", repository_routes())
        .nest("/product", product_routes())
        .nest("/inventory", inventory_routes())
        .nest("/order", order_routes())
        .route("/health", get(health))
        .with_state(pool.clone());

    env_logger::init();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081").await.unwrap();
    log::info!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

pub async fn health() -> &'static str {
    "OK"
}