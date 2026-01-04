use std::time::Duration;

use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};

use rdkafka::ClientConfig;

pub struct KafkaProducer {
    future_producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new() -> Self {
        let mut config = ClientConfig::new();
        config.set("bootstrap.servers", "localhost:9092");

        let producer: FutureProducer = config.create().expect("failed to create future producer");

        Self {
            future_producer: producer,
        }
    }

    pub async fn produce_kafka_message(&self, topic: &str, msg: String) {
        let record = FutureRecord::to(topic)
            .payload(msg.as_str())
            .key("S3CrEt");

        let status_delivery = self
            .future_producer
            .send(record, Timeout::After(Duration::from_secs(3)))
            .await;

        match status_delivery {
            Ok(msg) => {
                tracing::info!(message = ?msg, "success: ")
            }
            Err(e) => {
                tracing::error!(error = ?e, "error send msg: ")
            }
        }
    }
}
