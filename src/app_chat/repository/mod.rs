use axum::async_trait;
use uuid::Uuid;

use crate::app_chat::domain::error::ChatError;

#[async_trait]
pub trait ChatRepository: Send + Sync {
    async fn chat(
        &self,
        qdrant_collection: &str,
        user_id: &Uuid,
        document_id: &Uuid,
        question: &str,
    ) -> Result<String, ChatError>;
}
