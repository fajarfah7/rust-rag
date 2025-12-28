use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::document_parser::{
    domain::error::PdfParserError, infrastructure::vector_storage::qdrant_upsert::QdrantPayload,
};

#[derive(Debug, Deserialize)]
pub struct QdrantSearchResponse {
    result: Vec<QdrantScorePoint>,
}

#[derive(Debug, Deserialize)]
pub struct QdrantScorePoint {
    score: f32,
    payload: QdrantPayload,
}

pub async fn search_qdrant(
    collection: &str,
    query_vector: Vec<f32>,
    limit: usize,
) -> Result<Vec<(QdrantPayload, f32)>, PdfParserError> {
    let client = Client::new();
    let url = format!(
        "http://localhost:6333/collections/{}/points/search",
        collection
    );

    let res = client
        .post(url)
        .json(&json!({
            "vector": query_vector,
            "limit": limit,
            "with_payload": true,
        }))
        .send()
        .await
        .map_err(|e| PdfParserError::UnknownError(format!("qdrant_search 1: {}", e.to_string())))?
        .error_for_status()
        .map_err(|e| PdfParserError::UnknownError(format!("qdrant_search 2: {}", e.to_string())))?;

    let body: QdrantSearchResponse = res
        .json()
        .await
        .map_err(|e| PdfParserError::UnknownError(format!("qdrant_search 3: {}", e.to_string())))?;

    Ok(body
        .result
        .into_iter()
        .map(|p| (p.payload, p.score))
        .collect())
}
