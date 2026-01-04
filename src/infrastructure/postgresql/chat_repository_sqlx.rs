use axum::async_trait;
use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::app_chat::domain::chat_message::Message;
use crate::app_chat::{
    domain::chat_conversation::Conversation, repository::repository_chat::ChatRepository,
};
use crate::request::pagination::PaginationRequest;

#[derive(Debug)]
pub struct ChatRepositorySqlx {
    pool: PgPool,
}

impl ChatRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChatRepository for ChatRepositorySqlx {
    // CONVERSATION
    async fn count_conversations(&self, user_id: &Uuid) -> Result<i64, sqlx::Error> {
        let mut qb = QueryBuilder::new(
            r#"
            SELECT COUNT(id) FROM conversations
            "#,
        );

        qb.push(" WHERE user_id = ").push_bind(user_id);

        let total: i64 = qb.build_query_scalar().fetch_one(&self.pool).await?;

        Ok(total)
    }

    async fn find_conversations(
        &self,
        user_id: &Uuid,
        req: &PaginationRequest,
    ) -> Result<Vec<Conversation>, sqlx::Error> {
        let mut qb = QueryBuilder::new(
            r#"
            SELECT id, user_id, title, summary, created_at, updated_at FROM conversations
            "#,
        );

        qb.push(" WHERE user_id = ").push_bind(user_id);

        if let Some(s) = req.format_sort() {
            qb.push(" ORDER BY ").push(format!("{}", s));
        }

        qb.push(" LIMIT ")
            .push_bind(req.per_page.unwrap_or(10) as i64);

        if let Some(o) = req.offset {
            qb.push(" OFFSET ").push_bind(o as i64);
        }

        let conversations = qb
            .build_query_as::<Conversation>()
            .fetch_all(&self.pool)
            .await?;

        Ok(conversations)
    }

    async fn find_conversation_by_id(
        &self,
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<Conversation>, sqlx::Error> {
        let conversation = sqlx::query_as!(
            Conversation,
            r#"
            SELECT id, user_id, title, summary, created_at, updated_at 
            FROM conversations
            WHERE user_id = $1 AND id = $2
            "#,
            user_id,
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Some(conversation))
    }

    async fn create_conversation(
        &self,
        conversation: Conversation,
    ) -> Result<Conversation, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO conversations (id, user_id, title, summary, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, title, created_at, updated_at
            "#,
            conversation.id,
            conversation.user_id,
            conversation.title,
            conversation.summary,
            conversation.created_at,
            conversation.updated_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(conversation)
    }

    async fn update_conversation_title(&self, id: &Uuid, title: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE conversations 
            SET title = $1
            WHERE id = $2
            "#,
            title,
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_conversation_summary(&self, id: &Uuid, summary: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE conversations 
            SET summary = $1
            WHERE id = $2
            "#,
            summary,
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_conversation(&self, id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM conversations 
            WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // MESSAGE
    async fn count_messages(&self, conversation_id: &Uuid) -> Result<i64, sqlx::Error> {
        let mut qb = QueryBuilder::new(
            r#"
            SELECT COUNT(id) 
            FROM messages
            "#,
        );

        qb.push(" WHERE conversation_id = ").push_bind(conversation_id);

        let total: i64 = qb.build_query_scalar().fetch_one(&self.pool).await?;

        Ok(total)
    }

    async fn find_messages(
        &self,
        conversation_id: &Uuid,
        req: &PaginationRequest,
    ) -> Result<Vec<Message>, sqlx::Error> {
        let mut qb = QueryBuilder::new(
            r#"
            SELECT id, conversation_id, role, content, created_at
            FROM messages
            "#,
        );

        qb.push(" WHERE conversation_id = ")
            .push_bind(conversation_id);

        if let Some(s) = req.format_sort() {
            qb.push(" ORDER BY ").push(s);
        }

        if let Some(l) = req.per_page {
            qb.push(" LIMIT ").push_bind(l as i64);
        }

        if let Some(o) = req.offset {
            qb.push(" OFFSET ").push_bind(o as i64);
        }

        let messages = qb.build_query_as().fetch_all(&self.pool).await?;

        Ok(messages)
    }

    async fn create_message(&self, message: Message) -> Result<Message, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO messages(id, conversation_id, role, content, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, conversation_id, role, content, created_at
            "#,
            message.id,
            message.conversation_id,
            message.role,
            message.content,
            message.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(message)
    }

    async fn delete_message_by_conversation(
        &self,
        conversation_id: &Uuid,
    ) -> Result<(), sqlx::Error>{
        sqlx::query!(
            r#"
            DELETE FROM messages 
            WHERE conversation_id = $1
            "#,
            conversation_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
