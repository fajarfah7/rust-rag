use std::sync::Arc;

use axum::Router;
use axum::{middleware, routing::{post, get}};
use reqwest::Client;
use sqlx::PgPool;

use crate::app_chat::handler::handler_chat::{get_chat_messages, get_chats};
use crate::{
    app_chat::{handler::handler_chat::chat, usecase::usecase_chat::ChatUsecase},
    infrastructure::postgresql::chat_repository_sqlx::ChatRepositorySqlx,
    middleware::atuh_middleware::auth_middleware,
    repository::{embedder::embedder_lm_studio::LmStudioEmbedder, llm::lm_studio::LmStudio},
};

pub fn chat_route(pool: PgPool) -> Router {
    let repo = ChatRepositorySqlx::new(pool);
    let embedder = LmStudioEmbedder {
        base_url: "http://localhost:1234".into(),
        model: "nomic-ai/nomic-embed-text-v1.5-GGUF".into(),
    };
    let client = Client::new();
    let llm = LmStudio::new(
        client,
        "http://localhost:1234".into(),
        "meta-llama-3-8b-instruct".into(),
    );
    let usecase = Arc::new(ChatUsecase::new(repo, embedder, llm));

    Router::new()
        .route("/chat", post(chat))
        .route("/chat", get(get_chats))
        .route("/chat/:id", get(get_chat_messages))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(usecase)
}
