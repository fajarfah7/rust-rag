use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http,
};
use serde::de::DeserializeOwned;

use crate::response::error::ResponseError;

pub struct SafeQuery<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for SafeQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ResponseError;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Query(t) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|_| return ResponseError::BadRequest("invalid query param".into()))?;

        Ok(SafeQuery(t))
    }
}
