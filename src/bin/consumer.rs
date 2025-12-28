use futures::StreamExt;
use rag::app_document::domain::document_domain::Document;
use rag::config::environment::EnvConfig;
use rag::config::minio::new_minio_storage;
use rag::document_parser::infrastructure::embedder::embedder_lm_studio::LmStudioEmbedder;
use rag::document_parser::infrastructure::pdfium::loader::PdfLoader;
use rag::document_parser::infrastructure::vector_storage::qdrant_upsert::upsert_to_qdrant;
use rag::document_parser::usecase::embed_chunks::embed_chunks;
use rag::document_parser::usecase::ingest_pdf::IngestPdf;
use rag::infrastructure::storage::domain::FileStorage;
use rag::infrastructure::storage::minio::storage::FileStorageMinio;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    rag::init_env();
    rag::init_tracing();
    tracing::info!("STARTING");

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

    // INIT CONFIG FROM ENV
    let cfg = EnvConfig::init();
    // INIT CONFIG FROM ENV

    let qdrant_collection = cfg.qdrant_collection;

    // INIT STORAGE
    let s3 = new_minio_storage(
        &cfg.storage_region,
        &cfg.storage_access_key,
        &cfg.storage_secret_key,
        &cfg.storage_endpoint,
    )
    .await;
    let storage = FileStorageMinio::new(s3);

    // INIT CONSUMER
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "document-parser-consumer")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("consumer error");
    consumer.subscribe(&["document-parser"]).unwrap();

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
        let chunks = IngestPdf::execute(&doc, &file_path);

        tracing::info!("EMBED CHUNKS");
        let vectors = match embed_chunks(&embedder, &chunks).await {
            Ok(vec) => vec,
            Err(e) => {
                tracing::error!(error = ?e, "failed embed chunks");
                continue;
            }
        };

        tracing::info!("STORE TO QDRANT");
        match upsert_to_qdrant(&qdrant_collection, &chunks, &vectors).await {
            Ok(_) => {
                tracing::info!("DOCUMENT SUCCESSFULLY PROCESSED");
            }
            Err(e) => {
                tracing::error!(error = ?e, "failed store data to qdrant");
                continue;
            }
        }

        match tokio::fs::remove_file(&file_path).await {
            Ok(_) => {
                tracing::info!("DOCUMENT SUCCESSFULLY CLEARED");
            }
            Err(e) => {
                tracing::error!(error = ?e, "failed remove file");
                continue;
            }
        }

        tracing::info!("FINISHED");
        // STARTING EMBEDDING
    }
}
