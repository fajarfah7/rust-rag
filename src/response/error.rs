use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ResponseErrorBody {
    status: u16,
    message: String,
    detail: Option<String>,
}

#[derive(Debug)]
pub enum ResponseError {
    BadRequest(String),
    NotFound(String),
    DatabaseError,
    Unauthorized,
    InvalidToken,
    InternalServerError,
    Multipart(axum::extract::multipart::MultipartError),
}
impl From<axum::extract::multipart::MultipartError> for ResponseError {
    fn from(err: axum::extract::multipart::MultipartError) -> Self {
        ResponseError::Multipart(err)
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        match self {
            ResponseError::BadRequest(msg) => (StatusCode::BAD_REQUEST, {
                let body = ResponseErrorBody {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: msg,
                    detail: None,
                };
                Json(body)
            })
                .into_response(),
            ResponseError::NotFound(msg) => (StatusCode::NOT_FOUND, {
                let body = ResponseErrorBody {
                    status: StatusCode::NOT_FOUND.as_u16(),
                    message: msg,
                    detail: None,
                };
                Json(body)
            })
                .into_response(),
            ResponseError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, {
                let body = ResponseErrorBody {
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "internal server error".into(),
                    detail: Some("critical storage error".into()),
                };
                Json(body)
            })
                .into_response(),
            ResponseError::Unauthorized => (StatusCode::UNAUTHORIZED, {
                let body = ResponseErrorBody {
                    status: StatusCode::UNAUTHORIZED.as_u16(),
                    message: "unauthorized".into(),
                    detail: None,
                };
                Json(body)
            })
                .into_response(),
            ResponseError::InvalidToken => (StatusCode::UNAUTHORIZED, {
                let body = ResponseErrorBody {
                    status: StatusCode::UNAUTHORIZED.as_u16(),
                    message: "invalid token".into(),
                    detail: None,
                };
                Json(body)
            })
                .into_response(),
            ResponseError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, {
                let body = ResponseErrorBody {
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "internal server error".into(),
                    detail: None,
                };
                Json(body)
            })
                .into_response(),
            ResponseError::Multipart(e) => (StatusCode::INTERNAL_SERVER_ERROR, {
                let body = ResponseErrorBody {
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "internal server error".into(),
                    detail: Some(e.body_text().to_string()),
                };
                Json(body)
            })
                .into_response(),
        }
    }
}
