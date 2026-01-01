use axum::async_trait;

use crate::{
    domain::chunk::Chunk, error::pdf_parser::PdfParserError,
    request::qdrant::request_qdrant::QdrantPayload,
};

#[async_trait]
pub trait VectorStorage {
    async fn search(
        &self,
        vector: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<(QdrantPayload, f32)>, PdfParserError>;

    async fn upsert_to_vector_storage(
        &self,
        owner_user_id: &str,
        document_id: &str,
        chunks: &[Chunk],
        vectors: &[Vec<f32>],
    ) -> Result<(), PdfParserError>;
}
