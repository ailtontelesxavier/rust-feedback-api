use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Json, Router};
use dotenv::dotenv;

use sqlx::{
    postgres::PgPoolOptions,
    Pool, Postgres,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::trace::TraceLayer;


pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Carrega as variáveis de ambiente do arquivo .env

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    // Inicializa o subscriber de logs
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new("tower_http=debug,info"))
        .init();

    let app = Router::new().route("/api/healthchecker", get(health_checker_handler))
    .layer(TraceLayer::new_for_http()); // Middleware de log;
    
    println!("Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Feedback CRUD API with Rust, SQLX, Postgres and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });
    
    Json(json_response)
}