use axum::async_trait;
use uuid::Uuid;

use crate::app_document::domain::document_domain::Document;

#[async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn find_document_by_id(&self,user_id: &Uuid, id: &Uuid) -> Result<Option<Document>, sqlx::Error>;
    async fn create_document(&self, document: Document) -> Result<Document, sqlx::Error>;
    async fn update_document_status(&self, id: &Uuid, status: &str) -> Result<(), sqlx::Error>;
}