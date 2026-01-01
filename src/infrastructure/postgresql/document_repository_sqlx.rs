use axum::async_trait;
use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::app_document::{
    domain::document_domain::Document, repository::document_repository::DocumentRepository,
};
use crate::request::pagination::PaginationRequest;

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
    async fn find_document_by_id(
        &self,
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<Document>, sqlx::Error> {
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

    async fn count_documents(&self, user_id: &Uuid) -> Result<i64, sqlx::Error> {
        let mut qb = QueryBuilder::new(
            r#"
            SELECT COUNT(id) 
            FROM documents
            "#
        );

        qb.push(" WHERE user_id = ").push_bind(user_id);

        let total: i64 = qb.build_query_scalar().fetch_one(&self.pool).await?;

        Ok(total)
    }
    
    async fn get_documents(
        &self,
        user_id: &Uuid,
        pagination: &PaginationRequest,
    ) -> Result<Vec<Document>, sqlx::Error> {
        let mut qb = QueryBuilder::new(
            r#"
            SELECT id, user_id, original_filename, storage_path, status, created_at, updated_at
            FROM documents
            "#
        );
        
        qb.push(" WHERE user_id = ").push_bind(user_id);

        if let Some(s) = pagination.format_sort() {
            qb.push(" ORDER BY ").push(format!("{}", s));
        }

        qb.push(" LIMIT ")
            .push_bind(pagination.per_page.unwrap_or(1) as i64);
        
        if let Some(o) = pagination.offset {
            qb.push(" OFFSET ").push_bind(o as i64);
        }

        let documents = qb
            .build_query_as::<Document>()
            .fetch_all(&self.pool)
            .await?;

        Ok(documents)
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
