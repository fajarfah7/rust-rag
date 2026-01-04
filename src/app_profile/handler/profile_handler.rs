use std::{str::FromStr, sync::Arc};

use axum::{Extension, extract::State, response::IntoResponse};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{app_profile::{repository::profile_repository::ProfileRepository, usecase::profile_usecase::ProfileUsecase}, middleware::jwt_token::claims::Claims, response::{error::ResponseError, success::ResponseSuccess}};

pub async fn get_profile<R: ProfileRepository>(
    State(usecase): State<Arc<ProfileUsecase<R>>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, ResponseError> {
    let user_id = Uuid::from_str(&claims.sub).map_err(|e| {
        tracing::error!(error = ?e, "failed parse uuid");
        return ResponseError::Unauthorized
    })?;

    let profile = usecase
    .get_profile(&user_id)
    .await?;

    Ok(ResponseSuccess::Object(StatusCode::OK, Some(profile)))
}