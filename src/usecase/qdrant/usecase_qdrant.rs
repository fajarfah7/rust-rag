use uuid::Uuid;
use reqwest::Client;
use serde_json::json;

use crate::{request::qdrant::request_qdrant::QdrantPayload, response::qdrant::response_qdrant::QdrantSearchResponse};
use crate::error::pdf_parser::PdfParserError;

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

    let res = client
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

pub async fn api_search_qdrant(
    collection: &str,
    query_vector: Vec<f32>,
    limit: usize,
    user_id: &Uuid,
    document_id: &Uuid,
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
            "filter": {
                "must": [
                    {
                        "key": "owner_user_id",
                        "match": {
                            "value": user_id.to_string(),
                        }
                    },
                    {
                        "key": "document_id",
                        "match": {
                            "value": document_id.to_string(),
                        }
                    }
                ]
            }
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
