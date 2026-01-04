use std::sync::Arc;
use aws_sdk_s3::Client;
use axum::{Router, extract::DefaultBodyLimit, middleware, routing::{get, post}};

use sqlx::PgPool;

use crate::middleware::atuh_middleware::auth_middleware;
use crate::{app_document::handler::document_handler::{get_document, get_documents, upload_document}, infrastructure::storage::minio::storage::FileStorageMinio};
use crate::{app_document::usecase::document_usecase::DocumentUsecase, infrastructure::{postgresql::document_repository_sqlx::DocumentRepositorySqlx}};

pub fn document_route(pool: PgPool, s3: Client) -> Router {
    let repo = DocumentRepositorySqlx::new(pool);
    let storage = FileStorageMinio::new(s3);
    let usecase = Arc::new(DocumentUsecase::new(repo, storage));

    Router::new()
    .route("/document", get(get_documents))
    .route("/document/:id", get(get_document))
    .route("/document/upload", post(upload_document))
    .layer(DefaultBodyLimit::max(30 * 1024 * 1024))
    .layer(middleware::from_fn(auth_middleware))
    .with_state(usecase)
}