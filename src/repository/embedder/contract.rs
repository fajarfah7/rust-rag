use axum::async_trait;

use crate::{domain::chunk::Chunk, error::embedder::EmbedError};

#[async_trait]
pub trait Embedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbedError>;
    async fn embed_chunks(&self, chunks: &[Chunk]) -> Result<Vec<Vec<f32>>, EmbedError>;
}