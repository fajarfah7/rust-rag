use std::{str::FromStr, sync::Arc};

use axum::{
    Extension, extract::{Multipart, Path, Query, State}, http::StatusCode, response::IntoResponse
};
use uuid::{Uuid};

use crate::{
    app_document::{
        repository::document_repository::DocumentRepository,
        usecase::{document_usecase::DocumentUsecase, types::UploadFileRequest},
    }, infrastructure::storage::domain::FileStorage, middleware::jwt_token::claims::Claims, request::pagination::PaginationRequest, response::{error::ResponseError, success::ResponseSuccess}
};

pub async fn get_document<R: DocumentRepository, S: FileStorage>(
    State(usecase): State<Arc<DocumentUsecase<R,S>>>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, ResponseError> {
    let user_id = Uuid::parse_str(&claims.sub)
    .map_err(|e| {
        tracing::error!(error = ?e, "failed parse uuid user");
        return ResponseError::InternalServerError
    })?;

    let doc_id = Uuid::parse_str(&id)
    .map_err(|e| {
        tracing::error!(error = ?e, "failed parse uuid id");
        return ResponseError::BadRequest("invalid data id".into())
    })?;

    let doc = usecase
    .find_document_by_id(&user_id, &doc_id)
    .await?;

    Ok(ResponseSuccess::Object(StatusCode::OK, Some(doc)))
}

pub async fn get_documents<R: DocumentRepository, S: FileStorage>(
    State(usecase): State<Arc<DocumentUsecase<R, S>>>,
    Extension(claims): Extension<Claims>,
    Query(q): Query<PaginationRequest>,
) -> Result<impl IntoResponse, ResponseError> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|e| {
        tracing::error!(error = ?e, "failed parse uuid");
        return ResponseError::InternalServerError
    })?;

    let page = match q.page {
        Some(p) => p,
        None => 1,
    };
    let per_page = match q.per_page {
        Some(pp) => pp,
        None => 10,
    };
    let offset = (page - 1) * per_page as u32;
    let sort = match q.sort {
        Some(s) => s,
        None => "created_at".into(),
    };

    let query = PaginationRequest {
        page: Some(page),
        per_page: Some(per_page),
        offset: Some(offset),
        search: Some("".into()),
        sort: Some(sort),
    };

    let document_data = usecase
    .get_documents(&user_id, &query)
    .await?;

    Ok(ResponseSuccess::Pagination(
        page, 
        per_page, 
        document_data.total_data as u64, 
        Some(document_data.data),
    ))
}

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
