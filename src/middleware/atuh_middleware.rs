use axum::{
    body::Body,
    http::{Request, header::AUTHORIZATION},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{
    middleware::jwt_token::{
        jwt::verify_token,
    },
    response::error::ResponseError,
};

pub async fn auth_middleware(mut req: Request<Body>, next: Next) -> Response {
    let auth = match req.headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
    {
        Some(v) => v,
        None => return ResponseError::Unauthorized.into_response(),
    };

    let token = match auth.strip_prefix("Bearer ") {
        Some(t) => t,
        None => return ResponseError::Unauthorized.into_response(),
    };

    let claims = match verify_token(token) {
        Ok(c) => c,
        Err(_) => return ResponseError::InvalidToken.into_response(),
    };

    // inject to request
    req.extensions_mut().insert(claims);

    next.run(req).await
}
