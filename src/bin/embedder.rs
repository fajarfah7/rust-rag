use futures::StreamExt;
use rag::app_document::domain::document_domain::Document;
use rag::app_document::repository::document_repository::DocumentRepository;
use rag::config::environment::EnvConfig;
use rag::config::kafka_consumer::create_kafka_consumer;
use rag::config::minio::new_minio_storage;
use rag::config::postgre::new_pg_pool;
use rag::infrastructure::pdfium::loader::PdfLoader;
use rag::infrastructure::postgresql::document_repository_sqlx::DocumentRepositorySqlx;
use rag::infrastructure::storage::domain::FileStorage;
use rag::infrastructure::storage::minio::storage::FileStorageMinio;
use rag::repository::embedder::contract::Embedder;
use rag::repository::embedder::embedder_lm_studio::LmStudioEmbedder;
use rag::repository::vector_storage::contract::VectorStorage;
use rag::repository::vector_storage::qdrant::QdrantVectorStorage;
use rag::usecase::ingest_pdf::usecase_ingest_pdf::IngestPdf;
use rdkafka::{Message};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use reqwest::Client;

#[tokio::main]
async fn main() {
    rag::init_env();
    rag::init_tracing();
    tracing::info!("CONSUMER EMBEDDING RUNNING");

    // INIT CONFIG FROM ENV
    let cfg = EnvConfig::init();
    let qdrant_collection = cfg.qdrant_collection;

    let pdf_loader = match PdfLoader::new() {
        Ok(pl) => pl,
        Err(e) => {
            tracing::error!(error = ?e, "failed init pdf loader");
            return;
        }
    };

    let embedder = LmStudioEmbedder {
        base_url: "http://localhost:1234".into(),
        model: "nomic-ai/nomic-embed-text-v1.5-GGUF".into(),
    };

    let pool = new_pg_pool(&cfg.database).await;
    let repo_doc = DocumentRepositorySqlx::new(pool);

    // INIT FILE STORAGE
    let s3 = new_minio_storage(
        &cfg.storage_region,
        &cfg.storage_access_key,
        &cfg.storage_secret_key,
        &cfg.storage_endpoint,
    )
    .await;
    let storage = FileStorageMinio::new(s3);

    // INIT VECTOR STORAGE
    let client = Client::new();
    let qdrant_vector_storage = QdrantVectorStorage::new(client, qdrant_collection);

    // INIT CONSUMER
    let group_id = "document-parser-consumer";
    let topics = &["document-parser"];
    let consumer = create_kafka_consumer(group_id, topics);

    let mut stream = consumer.stream();
    while let Some(message) = stream.next().await {
        let m = match message {
            Ok(m) => m,
            Err(e) => {
                tracing::error!(error = ?e, "kafka error");
                continue;
            }
        };

        tracing::info!("PAYLOAD RECEIVED");
        let payload = match m.payload_view::<str>() {
            Some(Ok(p)) => p,
            _ => continue,
        };

        tracing::info!("PARSING PAYLOAD");
        let document: Document = match serde_json::from_str(payload) {
            Ok(d) => d,
            Err(e) => {
                tracing::error!(error = ?e, "invalid message payload");
                continue;
            }
        };

        tracing::info!("PROCESSING FILE STARTED");
        let file_bytes = match storage.download_file(&document.storage_path).await {
            Ok(f) => f,
            Err(e) => {
                tracing::error!(error = ?e, "failed download file");
                continue;
            }
        };

        let file_path = format!("tmp/{}.pdf", document.id);

        let mut file = match fs::File::create(&file_path).await {
            Ok(f) => f,
            Err(e) => {
                tracing::error!(error = ?e, "failed create file");
                continue;
            }
        };

        match file.write_all(&file_bytes).await {
            Ok(_) => {
                tracing::info!("success write file")
            }
            Err(e) => {
                tracing::error!(error = ?e, "failed write file");
                continue;
            }
        }
        tracing::info!("PROCESSING FILE FINISHED");

        tracing::info!("LOAD PDF LOADER");
        let doc = match pdf_loader.load(&file_path) {
            Ok(d) => d,
            Err(e) => {
                tracing::error!(error = ?e, "failed load file");
                continue;
            }
        };

        tracing::info!("EXECUTING FILE");
        let chunks = IngestPdf::execute(&doc, &document.original_filename);

        tracing::info!("EMBED CHUNKS");
        let vectors = match embedder.embed_chunks(&chunks).await {
            Ok(vec) => vec,
            Err(e) => {
                tracing::error!(error = ?e, "failed embed chunks");
                continue;
            }
        };

        match qdrant_vector_storage.upsert_to_vector_storage(
            &document.user_id.to_string(),
            &document.id.to_string(),
            &chunks,
            &vectors,
        )
        .await
        {
            Ok(_) => {
                tracing::info!("DOCUMENT SUCCESSFULLY EXTRACTED");
            }
            Err(e) => {
                tracing::error!(error = ?e, "failed store data to qdrant");
                continue;
            }
        }

        match tokio::fs::remove_file(&file_path).await {
            Ok(_) => {
                tracing::info!("DOCUMENT SUCCESSFULLY CLEARED FROM TMP");
            }
            Err(e) => {
                tracing::error!(error = ?e, "failed remove file");
                continue;
            }
        }

        match repo_doc
            .update_document_status(&document.id, "success")
            .await
        {
            Ok(_) => {
                tracing::info!("FINISHED PROCESS DOCUMENT");
            }
            Err(e) => {
                tracing::error!(error = ?e, "failed update document status");
                continue;
            }
        }

        tracing::info!("FINISHED");
    }
}
