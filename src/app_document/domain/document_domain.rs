use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: Uuid,
    pub user_id: Uuid,
    pub original_filename: String,
    pub storage_path: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}