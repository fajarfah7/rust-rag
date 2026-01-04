use chrono::Utc;
use uuid::Uuid;

use crate::{
    app_chat::{
        domain::{chat_conversation::Conversation, chat_message::Message},
        repository::repository_chat::ChatRepository,
        usecase::types::{
            CreateChatResponse, GetConversationMessages, GetConversations,
        },
    },
    helper::builder::{build_context, build_prompt_v2},
    infrastructure::producer::produce::KafkaProducer,
    repository::{embedder::contract::Embedder, llm::contract::Llm},
    request::pagination::PaginationRequest,
    response::error::ResponseError,
    usecase::qdrant::usecase_qdrant::api_search_qdrant,
};

#[derive(Debug)]
pub struct ChatUsecase<R: ChatRepository, E: Embedder, L: Llm> {
    repo: R,
    embedder: E,
    llm: L,
}

impl<R: ChatRepository, E: Embedder, L: Llm> ChatUsecase<R, E, L> {
    pub fn new(repo: R, embedder: E, llm: L) -> Self {
        Self {
            repo,
            embedder,
            llm,
        }
    }

    pub async fn get_chats(
        &self,
        user_id: &Uuid,
        req: &PaginationRequest,
    ) -> Result<GetConversations, ResponseError> {
        let total_conversation = self.repo.count_conversations(user_id).await.map_err(|e| {
            tracing::error!(error = ?e, "failed count conversation");
            return ResponseError::DatabaseError;
        })?;

        let conversations = self
            .repo
            .find_conversations(user_id, req)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "failed find conversations");
                return ResponseError::DatabaseError;
            })?;

        Ok(GetConversations {
            data: conversations,
            total_data: total_conversation,
        })
    }

    pub async fn get_chat_messages(
        &self,
        user_id: &Uuid,
        conversation_id: &Uuid,
        req: &PaginationRequest,
    ) -> Result<GetConversationMessages, ResponseError> {
        let find_conversation = self
            .repo
            .find_conversation_by_id(user_id, conversation_id)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "failed find conversation");
                return ResponseError::DatabaseError;
            })?;

        let conversation = match find_conversation {
            Some(conv) => conv,
            None => return Err(ResponseError::NotFound("data not found".into())),
        };

        let total_messages = self
            .repo
            .count_messages(&conversation.id)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "failed count messages");
                return ResponseError::DatabaseError;
            })?;

        let mut messages = self
            .repo
            .find_messages(&conversation.id, req)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "failed find messages");
                return ResponseError::DatabaseError;
            })?;
        messages.reverse();

        Ok(GetConversationMessages {
            data: messages,
            total_data: total_messages,
        })
    }

    pub async fn create_chat(
        &self,
        user_id: &Uuid,
        conversation_id: Option<Uuid>,
        chat_message: &str,
    ) -> Result<CreateChatResponse, ResponseError> {
        // DEFAULT CONFIG
        let vector_collection = "pdf_chunks";
        let vector_search_limit: usize = 5;
        let context_max_chars: usize = 5_000;

        let conversation = match conversation_id {
            Some(id) => {
                let find_conversation = self
                    .repo
                    .find_conversation_by_id(user_id, &id)
                    .await
                    .map_err(|e| {
                        tracing::error!(error = ?e, "error on database");
                        return ResponseError::DatabaseError;
                    })?;
                match find_conversation {
                    Some(c) => c,
                    None => {
                        return Err(ResponseError::NotFound("data not found".into()));
                    }
                }
            }
            None => {
                // let new_user_id =
                let conversation: Conversation = Conversation {
                    id: Uuid::new_v4(),
                    user_id: *user_id,
                    title: Some(chat_message.to_string()),
                    summary: Some(chat_message.to_string()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };

                let conversation =
                    self.repo
                        .create_conversation(conversation)
                        .await
                        .map_err(|e| {
                            tracing::error!(error = ?e, "error on database");
                            return ResponseError::DatabaseError;
                        })?;
                conversation
            }
        };

        let user_message = Message {
            id: Uuid::new_v4(),
            conversation_id: conversation.id,
            role: "user".to_string(),
            content: chat_message.to_string(),
            created_at: Utc::now(),
        };

        self.repo.create_message(user_message).await.map_err(|e| {
            tracing::error!(error = ?e, "failed store user message");
            return ResponseError::DatabaseError;
        })?;

        let query_vector = self.embedder.embed(chat_message).await.map_err(|_| {
            tracing::error!("failed embed message");
            return ResponseError::InternalServerError;
        })?;

        let result = api_search_qdrant(
            &vector_collection,
            query_vector,
            vector_search_limit,
            user_id,
        )
        .await
        .map_err(|_| {
            tracing::error!("failed search on qdrant");
            return ResponseError::InternalServerError;
        })?;

        let context = build_context(&result, context_max_chars);

        // let summary = match &conversation.summary {
        //     Some(s) => s.to_string(),
        //     None => String::from(""),
        // };
        let prompt: String = build_prompt_v2(&context, chat_message);

        let answer = self.llm.ask(&prompt).await.map_err(|_| {
            tracing::error!("failed ask llm");
            return ResponseError::InternalServerError;
        })?;

        let assistant_response = Message {
            id: Uuid::new_v4(),
            conversation_id: conversation.id,
            role: "assistant".to_string(),
            content: answer.clone(),
            created_at: Utc::now(),
        };
        self.repo
            .create_message(assistant_response)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "failed store assistant response");
                return ResponseError::DatabaseError;
            })?;

        // let total_message = self
        // .repo
        // .count_messages(&conversation.id)
        // .await
        // .map_err(|e| {
        //     tracing::error!(error = ?e, "failed count messages");
        //     return ResponseError::DatabaseError;
        // })?;

        // if total_message > 10 {
        let payload_kafka = serde_json::to_string(&conversation).map_err(|e| {
            tracing::error!(error = ?e, "conversation failed parse to string");
            return ResponseError::InternalServerError;
        })?;

        let topic = "conversation-summary";
        KafkaProducer::new()
            .produce_kafka_message(topic, payload_kafka)
            .await;
        // }

        Ok(CreateChatResponse {
            conversation_id: conversation.id,
            answer: answer,
        })
    }
}
