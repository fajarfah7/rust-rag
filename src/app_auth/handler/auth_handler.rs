use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    app_auth::{
        handler::{
            map_usecase_error::map_usecase_auth_error, types::LoginRequest,
            validate_request::validate_request,
        },
        repository::auth_repository::AuthRepository,
        usecase::auth_usecase::AuthUsecase,
    },
    response::{error::ResponseError, success::ResponseSuccess},
};

pub async fn login<R: AuthRepository>(
    State(usecase): State<Arc<AuthUsecase<R>>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, ResponseError> {
    validate_request(&req)?;

    let token = usecase
        .login(&req.username, &req.password)
        .await
        .map_err(map_usecase_auth_error)?;

    Ok(ResponseSuccess::Object(StatusCode::OK, Some(token)))
}
