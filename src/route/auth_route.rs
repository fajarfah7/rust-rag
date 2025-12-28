use std::sync::Arc;

use axum::{Router, routing::{post}};
use sqlx::PgPool;

use crate::{app_auth::{handler::auth_handler::login, usecase::{auth_usecase::AuthUsecase}}, infrastructure::postgresql::auth_repository_sqlx::AuthRepositorySqlx};

pub fn auth_route(pool: PgPool) -> Router {
    let repo = AuthRepositorySqlx::new(pool);
    let usecase  = Arc::new(AuthUsecase::new(repo));

    Router::new()
    .route("/login", post(login))
    .with_state(usecase)
}