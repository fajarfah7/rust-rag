// use tokio::net::TcpListener;

// use axum::Router;

// use crate::{
//     config::{environment::EnvConfig, minio::new_minio_storage, postgre::new_pg_pool},
//     route::{auth_route::auth_route, document_route::document_route},
// };

// mod app_auth;
// mod app_document;
// mod config;
// mod infrastructure;
// mod middleware;
// mod request;
// mod response;
// mod route;

// #[tokio::main]
// async fn main() {
//     dotenvy::dotenv().ok();
//     tracing_subscriber::fmt::init();

//     let cfg = EnvConfig::init();

//     let pool = new_pg_pool(&cfg.database).await;
//     let storage = new_minio_storage(
//         &cfg.storage_region,
//         &cfg.storage_access_key,
//         &cfg.storage_secret_key,
//         &cfg.storage_endpoint,
//     )
//     .await;

//     let app = Router::new()
//         .merge(auth_route(pool.clone()))
//         .merge(document_route(pool.clone(), storage));

//     let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

//     axum::serve(listener, app).await.unwrap();
// }
fn main() {
    println!("{}", "RAG".to_string())
}