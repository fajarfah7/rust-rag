use std::sync::Arc;
use aws_sdk_s3::Client;
use axum::{Router, middleware, routing::post};

use sqlx::PgPool;

use crate::middleware::atuh_middleware::auth_middleware;
use crate::{app_document::handler::document_handler::upload_document, infrastructure::storage::minio::storage::FileStorageMinio};
use crate::{app_document::usecase::document_usecase::DocumentUsecase, infrastructure::{postgresql::document_repository_sqlx::DocumentRepositorySqlx}};

pub fn document_route(pool: PgPool, s3: Client) -> Router {
    let repo = DocumentRepositorySqlx::new(pool);
    let storage = FileStorageMinio::new(s3);
    let usecase = Arc::new(DocumentUsecase::new(repo, storage));

    Router::new()
    .route("/document/upload", post(upload_document))
    .layer(middleware::from_fn(auth_middleware))
    .with_state(usecase)
}