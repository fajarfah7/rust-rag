use chrono::Utc;
use sanitize_filename::sanitize;
use uuid::Uuid;
// use tracing::info;

use crate::{
    app_document::{
        domain::document_domain::Document, repository::document_repository::DocumentRepository,
        usecase::types::UploadFileRequest,
    },
    infrastructure::{producer::produce::KafkaProducer, storage::domain::FileStorage},
    response::error::ResponseError,
};

#[derive(Debug, Clone)]
pub struct DocumentUsecase<R: DocumentRepository, S: FileStorage> {
    repo: R,
    storage: S,
}

impl<R: DocumentRepository, S: FileStorage> DocumentUsecase<R, S> {
    pub fn new(repo: R, storage: S) -> Self {
        Self { repo, storage }
    }

    pub async fn upload_document(&self, req: UploadFileRequest) -> Result<(), ResponseError> {
        // HANDLE STORE TO S3
        let sanitized_filename = sanitize(&req.original_filename);
        let ext = std::path::Path::new(&sanitized_filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let unique = Uuid::new_v4().to_string();
        let date = Utc::now().format("%Y%m%d");
        let stored_filename = if ext.is_empty() {
            format!("{}_{}", date, unique)
        } else {
            format!("{}_{}.{}", date, unique, ext)
        };

        let object_key = format!("documents/{}", stored_filename);
        self.storage
            .upload_file(&object_key, req.bytes)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    "minio upload failed"
                );
                ResponseError::InternalServerError
            })?;

        // HANDLE STORE DATA TO DB
        let document_status = String::from("uploaded");
        let document = Document {
            id: Uuid::new_v4(),
            user_id: req.user_id,
            original_filename: req.original_filename,
            storage_path: object_key,
            status: document_status,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let document = self.repo.create_document(document).await.map_err(|e| {
            tracing::error!(
                error = ?e,
                "database error"
            );
            ResponseError::DatabaseError
        })?;

        let payload_kafka = serde_json::to_string(&document).map_err(|e| {
            tracing::error!(
                error = ?e,
                "failed parse payload for kafka"
            );
            ResponseError::InternalServerError
        })?;

        KafkaProducer::new().produce_kafka_message(payload_kafka).await;

        Ok(())
    }
}
