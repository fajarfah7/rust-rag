use uuid::Uuid;

#[derive(Debug)]
pub struct UploadFileRequest {
    pub user_id: Uuid,
    pub original_filename: String,
    pub content_type: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub enum UploadFileError {
    Minio(String),
    InvalidInput
}