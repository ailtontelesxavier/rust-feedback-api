mod handler;
mod model;
mod route;
mod schema;


use std::sync::Arc;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};

use axum::{Extension};
use minijinja::{path_loader, Environment};
use dotenv::dotenv;
use route::create_router;
use tower_http::cors::CorsLayer;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use tracing_subscriber;
//use tower_http::trace::TraceLayer;
use tower_http::trace::{TraceLayer};
use tracing::{Level};


pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("🔥 Failed to connect to the database");

    // Inicializa o subscriber com base no RUST_LOG
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Carrega os arquivos da pasta ./templates/
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));

    // Compartilhado entre threads com Arc
    let env = Arc::new(env);

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool }))
        .layer(cors)
        .layer(Extension(env))
        // Adiciona o TraceLayer para rastreamento de requisições HTTP
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(tower_http::trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(tower_http::trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
