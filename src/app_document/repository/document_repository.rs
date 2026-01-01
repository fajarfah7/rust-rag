use axum::async_trait;
use uuid::Uuid;

use crate::app_document::domain::document_domain::Document;
use crate::request::pagination::PaginationRequest;
#[async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn find_document_by_id(
        &self,
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<Document>, sqlx::Error>;
    async fn count_documents(&self, user_id: &Uuid) -> Result<i64, sqlx::Error>;
    async fn get_documents(
        &self,
        user_id: &Uuid,
        pagination: &PaginationRequest,
    ) -> Result<Vec<Document>, sqlx::Error>;
    async fn create_document(&self, document: Document) -> Result<Document, sqlx::Error>;
    async fn update_document_status(&self, id: &Uuid, status: &str) -> Result<(), sqlx::Error>;
}
