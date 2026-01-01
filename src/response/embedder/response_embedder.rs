use serde::Deserialize;

use crate::domain::embedder::EmbeddingData;

#[derive(Debug, Deserialize)]
pub struct EmbeddingResponse {
    pub data: Vec<EmbeddingData>,
}