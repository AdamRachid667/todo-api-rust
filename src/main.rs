mod handlers;
mod models;
mod openapi;

// use openapi::ApiDoc;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};
// use models::{CompletedTodo, CreateTodo, Todo, UpdateTodo};
use sqlx::sqlite::SqlitePool;
use std::env;
// use utoipa::OpenApi; // Ensure utoipa is imported
// use utoipa_swagger_ui::SwaggerUi;

//use crate::openapi::openapi_spec; // Ensure swagger ui is imported
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    // Utilisation d'une variable d'environnement pour la configuration
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://db.sqlite".to_string());

    let server_address =
        env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Échec de la connexion à la base de données");

    sqlx::query("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY, task TEXT NOT NULL, completed BOOLEAN NOT NULL DEFAULT 0)")
        .execute(&pool)
        .await
        .expect("Échec de la migration initiale");

    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/todos", get(handlers::get_all))
        .route("/todos/{id}", get(handlers::get_one))
        .route("/todos", post(handlers::create))
        .route("/todos/{id}", patch(handlers::update))
        .route("/todos/{id}", delete(handlers::deletes))
        .route("/todos/{id}/completed", patch(handlers::set_completed))
        .route("/todos/import", post(handlers::multi_create))
        .route("/todos/search/{needle}", get(handlers::search_task))
        .route("/todos/import-csv", post(handlers::import_csv))
        .route("/todos/export", get(handlers::export_csv))
        //.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // .route(openapi::OPENAPI_URL, get(openapi::openapi_spec))
        // .route("/scalar", get(|| async { axum::response::Html(openapi::get_scalar_html()) }))        
        .with_state(pool);

    let app = app.merge(openapi::create_router());

    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .unwrap();
    println!("Sever started on: http://{}", server_address);
    axum::serve(listener, app).await.unwrap();
}
