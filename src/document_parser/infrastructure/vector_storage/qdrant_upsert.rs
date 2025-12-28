use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::document_parser::domain::{chunk::Chunk, error::PdfParserError};

#[derive(Debug, Serialize)]
pub struct QdrantPoint {
    pub id: u64,
    pub vector: Vec<f32>,
    pub payload: QdrantPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QdrantPayload {
    pub text: String,
    pub page: usize,
    pub source: String,
    pub index: usize,
}

pub async fn upsert_to_qdrant(
    collection: &str,
    chunks: &[Chunk],
    vectors: &[Vec<f32>],
) -> Result<(), PdfParserError> {
    let points: Vec<_> = chunks
        .iter()
        .zip(vectors.iter())
        .enumerate()
        .map(|(i, (chunk, vector))| QdrantPoint {
            id: i as u64,
            vector: vector.clone(),
            payload: QdrantPayload {
                text: chunk.text.clone(),
                page: chunk.page,
                source: chunk.source.clone(),
                index: chunk.index,
            },
        })
        .collect();

    let client = Client::new();
    let url = format!(
        "http://localhost:6333/collections/{}/points?wait=true",
        collection
    );

    client
        .put(url)
        .json(&json!({ "points": points }))
        .send()
        .await
        .map_err(|e| PdfParserError::UnknownError(e.to_string()))?
        .error_for_status()
        .map_err(|e| PdfParserError::UnknownError(e.to_string()))?;

    Ok(())
}
