use std::{str::FromStr, sync::Arc};

use axum::{
    Extension, extract::{Multipart, State}, http::StatusCode, response::IntoResponse
};
use uuid::{Uuid};

use crate::{
    app_document::{
        repository::document_repository::DocumentRepository,
        usecase::{document_usecase::DocumentUsecase, types::UploadFileRequest},
    }, infrastructure::storage::domain::FileStorage, middleware::jwt_token::claims::Claims, response::{error::ResponseError, success::ResponseSuccess}
};

pub async fn upload_document<R: DocumentRepository, S: FileStorage>(
    State(usecase): State<Arc<DocumentUsecase<R, S>>>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ResponseError> {
    let mut claims_user_id = String::new();

    let mut original_filename = String::new();
    let mut content_type = String::new();
    let mut bytes = Vec::new();

    if claims.sub != "" {
        claims_user_id = claims.sub
    }

    let user_id = match Uuid::from_str(&claims_user_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err(ResponseError::InvalidToken)
        },
    };

    while let Some(field) = multipart.next_field().await? {
        if field.name() == Some("file") {
            original_filename = field.file_name().unwrap_or("file").to_string();

            content_type = field
                .content_type()
                .unwrap_or("application/octet-stream")
                .to_string();

            bytes = field.bytes().await?.to_vec();
        }
    }
    
    let req = UploadFileRequest {
        user_id,
        original_filename,
        content_type,
        bytes,
    };

    usecase
        .upload_document(req)
        .await
        .map_err(|_| ResponseError::InternalServerError)?;

    Ok(ResponseSuccess::Object(
        StatusCode::CREATED,
        "success".into(),
    ))
}
