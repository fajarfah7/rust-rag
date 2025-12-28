use axum::async_trait;

use crate::infrastructure::storage::error::StorageError;

#[async_trait]
pub trait FileStorage {
    async fn upload_file(
        &self,
        key: &str,
        bytes: Vec<u8>,
    ) -> Result<(), StorageError>;

    async fn download_file(
        &self,
        key: &str,
    ) -> Result<Vec<u8>, StorageError>;

    // async fn presigned_get(
    //     &self,
    //     key: &str,
    // ) -> Result<String, StorageError>;
}
