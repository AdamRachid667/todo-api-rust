mod handlers;
mod models;
mod openapi;

use tower_http::cors::CorsLayer;

use axum::{
    Router,
    routing::{get, patch, post},
};

use sqlx::sqlite::SqlitePool;
use std::env;

use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://db.sqlite".to_string());

    let server_address =
        env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tasks (
        id INTEGER PRIMARY KEY, 
        task TEXT NOT NULL, 
        completed BOOLEAN NOT NULL DEFAULT 0,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        completed_at DATETIME
    )",
    )
    .execute(&pool)
    .await
    .expect("Échec de la migration initiale");

    let app = Router::new()
        .route("/", get(handlers::serve_root_page))
        .route("/tasks", get(handlers::get_all).post(handlers::create))
        .route("/tasks/export", get(handlers::export_csv))
        .route("/tasks/import", post(handlers::multi_create))
        .route("/tasks/import-csv", post(handlers::import_csv))
        .route("/tasks/search/{needle}", get(handlers::search_task))
        .route(
            "/tasks/{id}",
            get(handlers::get_one)
                .patch(handlers::update)
                .delete(handlers::deletes),
        )
        .route("/tasks/{id}/completed", patch(handlers::set_completed))
        .layer(CorsLayer::permissive())
        .fallback_service(ServeDir::new("static"))
        .with_state(pool)
        .merge(openapi::create_router());

    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .unwrap();
    println!("Sever started on: http://{}", server_address);
    axum::serve(listener, app).await.unwrap();
}
