use aws_sdk_s3::{Client, primitives::ByteStream};
use axum::async_trait;

use crate::infrastructure::storage::{domain::FileStorage, error::StorageError};

#[derive(Debug)]
pub struct FileStorageMinio {
    pub s3: Client,
    pub bucket: String,
}

impl FileStorageMinio {
    pub fn new(s3: Client) -> Self {
        let bucket = "rag".to_string();
        Self { s3, bucket }
    }
}

#[async_trait]
impl FileStorage for FileStorageMinio {
    async fn upload_file(&self, key: &str, bytes: Vec<u8>) -> Result<(), StorageError> {
        self.s3
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(bytes))
            .send()
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    bucket = %self.bucket,
                    key = %key,
                    "minio upload failed"
                );
                StorageError::StorageError(e.to_string())
            })?;

        Ok(())
    }

    async fn download_file(&self, key: &str) -> Result<Vec<u8>, StorageError> {
        let resp = self
            .s3
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    bucket = %self.bucket,
                    key = %key,
                    "minio upload failed"
                );
                StorageError::StorageError(e.to_string())
            })?;

        let agg_bytes = resp.body.collect().await.map_err(|e| {
            tracing::error!(
                error = ?e,
                bucket = %self.bucket,
                key = %key,
                "minio upload failed"
            );
            StorageError::StorageError(e.to_string())
        })?;
        let data = agg_bytes.into_bytes();

        Ok(data.to_vec())
    }

    // async fn presigned_get(&self, key: &str) -> Result<String, StorageError> {
    //     let presigned = self
    //         .s3
    //         .get_object()
    //         .bucket(self.bucket)
    //         .key(key)
    //         .presigned(Duration::from_hours(1))
    //         .await?;

    //     Ok(presigned.uri().to_string())
    // }
}
