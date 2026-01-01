use serde::Deserialize;

use crate::request::qdrant::request_qdrant::QdrantPayload;

#[derive(Debug, Deserialize)]
pub struct QdrantSearchResponse {
    pub result: Vec<QdrantScorePoint>,
}

#[derive(Debug, Deserialize)]
pub struct QdrantScorePoint {
    pub score: f32,
    pub payload: QdrantPayload,
}
