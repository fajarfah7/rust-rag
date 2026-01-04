use serde::Serialize;
use uuid::Uuid;

use crate::app_chat::domain::{chat_conversation::Conversation, chat_message::Message};

#[derive(Debug, Serialize)]
pub struct CreateChatResponse {
    pub conversation_id: Uuid,
    pub answer: String,
}

pub struct GetConversations {
    pub data: Vec<Conversation>,
    pub total_data: i64,
}

pub struct GetConversationMessages {
    pub data: Vec<Message>,
    pub total_data: i64,
}
