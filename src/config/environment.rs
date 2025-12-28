use std::env;

#[derive(Debug)]
pub struct EnvConfig {
    pub database: String,
    pub storage_bucket: String,
    pub storage_access_key: String,
    pub storage_secret_key: String,
    pub storage_region: String,
    pub storage_endpoint: String,
    pub qdrant_collection: String,
}

impl EnvConfig {
    pub fn init() -> Self {
        Self {
            database: env::var("DATABASE_URL").expect("database is not set"),
            storage_bucket: env::var("STORAGE_BUCKET").expect("storage bucket is not set"),
            storage_access_key: env::var("STORAGE_ACCESS_KEY")
                .expect("storage access key is not set"),
            storage_secret_key: env::var("STORAGE_SECRET_KEY")
                .expect("storage secret key is not set"),
            storage_endpoint: env::var("STORAGE_ENDPOINT").expect("storage endpoint is not set"),
            storage_region: env::var("STORAGE_REGION").expect("storage region is not set"),
            qdrant_collection: env::var("QDRANT_COLLECTION").expect("qdrant collection is not set"),
        }
    }
}
