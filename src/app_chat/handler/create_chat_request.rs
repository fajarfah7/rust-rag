use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateChatRequest {
    pub conversation_id: Option<Uuid>,
    pub message: String,
}