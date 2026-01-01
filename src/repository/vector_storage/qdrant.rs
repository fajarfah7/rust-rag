use axum::async_trait;
use reqwest::Client;
use serde_json::json;
use uuid::Uuid;

use crate::{
    domain::chunk::Chunk,
    error::pdf_parser::PdfParserError,
    repository::vector_storage::contract::VectorStorage,
    request::qdrant::request_qdrant::{QdrantPayload, QdrantPoint},
    response::qdrant::response_qdrant::QdrantSearchResponse,
};

pub struct QdrantVectorStorage {
    client: Client,
    collection: String,
}

impl QdrantVectorStorage {
    pub fn new(client: Client, collection: String) -> Self {
        Self { client, collection }
    }
}

#[async_trait]
impl VectorStorage for QdrantVectorStorage {
    async fn search(
        &self,
        query_vector: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<(QdrantPayload, f32)>, PdfParserError> {
        let url = format!(
            "http://localhost:6333/collections/{}/points/search",
            &self.collection
        );

        // "vector": query_vector,
        //         "limit": limit,
        //         "with_payload": true,
        //         "filter": {
        //             "must": [
        //                 {
        //                     "key": "city",
        //                     "match": {
        //                         "value": "London"
        //                     }
        //                 }
        //             ]
        //         }

        let res = self
            .client
            .post(url)
            .json(&json!({
                "vector": query_vector,
                "limit": limit,
                "with_payload": true,
                "filter": {
                    "must": [
                        {
                            "key": "city",
                            "match": {
                                "value": "London"
                            }
                        }
                    ]
                }
            }))
            .send()
            .await
            .map_err(|e| {
                PdfParserError::UnknownError(format!("qdrant_search 1: {}", e.to_string()))
            })?
            .error_for_status()
            .map_err(|e| {
                PdfParserError::UnknownError(format!("qdrant_search 2: {}", e.to_string()))
            })?;

        let body: QdrantSearchResponse = res.json().await.map_err(|e| {
            PdfParserError::UnknownError(format!("qdrant_search 3: {}", e.to_string()))
        })?;

        Ok(body
            .result
            .into_iter()
            .map(|p| (p.payload, p.score))
            .collect())
    }

    async fn upsert_to_vector_storage(
        &self,
        owner_user_id: &str,
        document_id: &str,
        chunks: &[Chunk],
        vectors: &[Vec<f32>],
    ) -> Result<(), PdfParserError> {
        let points: Vec<_> = chunks
            .iter()
            .zip(vectors.iter())
            .enumerate()
            .map(|(_, (chunk, vector))| QdrantPoint {
                id: Uuid::new_v4().to_string(),
                vector: vector.clone(),
                payload: QdrantPayload {
                    index: chunk.index,
                    page: chunk.page,
                    source: chunk.source.clone(),
                    text: chunk.text.clone(),
                    owner_user_id: owner_user_id.to_string(),
                    document_id: document_id.to_string(),
                },
            })
            .collect();

        let url = format!(
            "http://localhost:6333/collections/{}/points?wait=true",
            &self.collection
        );

        self.client
            .put(url)
            .json(&json!({ "points": points }))
            .send()
            .await
            .map_err(|e| PdfParserError::UnknownError(e.to_string()))?
            .error_for_status()
            .map_err(|e| PdfParserError::UnknownError(e.to_string()))?;

        Ok(())
    }
}
