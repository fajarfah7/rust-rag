use axum::{async_trait};
use sqlx::PgPool;
use uuid::Uuid;

use crate::app_document::{domain::document_domain::Document, repository::document_repository::DocumentRepository};

#[derive(Debug)]
pub struct DocumentRepositorySqlx {
    pool: PgPool,
}

impl DocumentRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DocumentRepository for DocumentRepositorySqlx {
    async fn find_document_by_id(&self, user_id: &Uuid, id: &Uuid) -> Result<Option<Document>, sqlx::Error> {
        let document = sqlx::query_as!(
            Document,
            r#"
            SELECT 
                id, 
                user_id, 
                original_filename,
                storage_path,
                status,
                created_at,
                updated_at
            FROM documents
            WHERE user_id = $1 AND id = $2
            "#,
            user_id,
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Some(document))
    }

    async fn create_document(&self, document: Document) -> Result<Document, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO documents (id, user_id, original_filename, storage_path, status, created_at, updated_at) 
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, original_filename, storage_path, status, created_at, updated_at
            "#,
            document.id,
            document.user_id,
            document.original_filename,
            document.storage_path,
            document.status,
            document.created_at,
            document.updated_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(document)
    }

    async fn update_document_status(&self, id: &Uuid, status: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE documents 
            SET status = $1 
            WHERE id = $2
            "#,
            status,
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
