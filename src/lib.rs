pub mod app_auth;
pub mod app_chat;
pub mod app_document;
pub mod config;
pub mod helper;
pub mod infrastructure;
pub mod middleware;
pub mod request;
pub mod response;
pub mod route;
pub mod domain;
pub mod usecase;
pub mod repository;
pub mod error;

pub fn init_env() {
    dotenvy::dotenv().ok();
}

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .try_init()
        .ok();
}
