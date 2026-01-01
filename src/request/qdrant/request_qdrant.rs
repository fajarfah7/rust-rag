use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct QdrantPoint {
    pub id: String,
    pub payload: QdrantPayload,
    pub vector: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QdrantPayload {
    pub index: usize,
    pub page: usize,
    pub source: String,
    pub text: String,
    pub document_id: String,
    pub owner_user_id: String,
}