use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
};
use uuid::Uuid;

use crate::response::error::ResponseError;

pub struct PathUuid(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for PathUuid
where
    S: Send + Sync,
{
    type Rejection = ResponseError;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Path(id) = Path::<String>::from_request_parts(parts, state)
            .await
            .map_err(|_| ResponseError::BadRequest("missing id".into()))?;

        let uuid =
            Uuid::parse_str(&id).map_err(|_| ResponseError::BadRequest("invalid uuid".into()))?;

        Ok(PathUuid(uuid))
    }
}
