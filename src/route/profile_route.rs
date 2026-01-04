use std::sync::Arc;

use axum::{Router, middleware};
use sqlx::PgPool;

use crate::{app_profile::{handler::profile_handler::get_profile, usecase::profile_usecase::ProfileUsecase}, infrastructure::postgresql::profile_repository_sqlx::ProfileRepositorySqlx, middleware::atuh_middleware::auth_middleware};

pub fn profile_route(pool: PgPool) -> Router {
    let repo = ProfileRepositorySqlx::new(pool);
    let usecase = Arc::new(ProfileUsecase::new(repo));

    Router::new()
    .route("/profile", axum::routing::get(get_profile))
    .layer(middleware::from_fn(auth_middleware))
    .with_state(usecase)
}