use tokio::net::TcpListener;

use axum::{Router, http::HeaderValue};

use rag::{
    config::{environment::EnvConfig, minio::new_minio_storage, postgre::new_pg_pool},
    route::{auth_route::auth_route, chat_route::chat_route, document_route::document_route, profile_route::profile_route},
};
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    rag::init_env();
    rag::init_tracing();
    // tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
    .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
    .allow_methods(Any)
    .allow_headers(Any);

    let cfg = EnvConfig::init();

    let pool = new_pg_pool(&cfg.database).await;
    let storage = new_minio_storage(
        &cfg.storage_region,
        &cfg.storage_access_key,
        &cfg.storage_secret_key,
        &cfg.storage_endpoint,
    )
    .await;

    let app = Router::new()
        .merge(auth_route(pool.clone()))
        .merge(document_route(pool.clone(), storage))
        .merge(chat_route(pool.clone()))
        .merge(profile_route(pool).clone())
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    
    tracing::info!("API RUNNING http://localhost:8080");

    axum::serve(listener, app).await.unwrap();
}
