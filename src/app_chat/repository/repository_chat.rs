use axum::async_trait;
use uuid::Uuid;

use crate::{app_chat::domain::{chat_conversation::Conversation, chat_message::Message}, request::pagination::PaginationRequest};

#[async_trait]
pub trait ChatRepository: Send + Sync {
    // CONVERSATION
    async fn count_conversations(&self, user_id: &Uuid) -> Result<i64, sqlx::Error>;
    async fn find_conversations(&self, user_id: &Uuid, req: &PaginationRequest) -> Result<Vec<Conversation>, sqlx::Error>;
    async fn find_conversation_by_id(&self, user_id: &Uuid, id: &Uuid) -> Result<Option<Conversation>, sqlx::Error>;
    async fn create_conversation(&self, conversation: Conversation) -> Result<Conversation, sqlx::Error>;
    async fn update_conversation_title(&self, id: &Uuid, title: &str) -> Result<(), sqlx::Error>;
    async fn update_conversation_summary(&self, id: &Uuid, summary: &str) -> Result<(), sqlx::Error>;
    async fn delete_conversation(&self, id: &Uuid) -> Result<(), sqlx::Error>;

    // MESSAGE
    async fn count_messages(&self, conversation_id: &Uuid) -> Result<i64, sqlx::Error>;
    async fn find_messages(&self, conversation_id: &Uuid, req: &PaginationRequest) -> Result<Vec<Message>, sqlx::Error>;
    async fn create_message(&self, message: Message) -> Result<Message, sqlx::Error>;
    async fn delete_message_by_conversation(&self, conversation_id: &Uuid) -> Result<(), sqlx::Error>;
}