use std::env;
use axum::Router;
use axum::routing::get;
use db_web::handlers::user::user_routes;
use sqlx::mysql::MySqlPoolOptions;

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
        .route("/", get(root))
        .with_state(pool.clone());

    env_logger::init();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081").await.unwrap();
    log::info!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

pub async fn root() -> &'static str {
    "Hello world"
}