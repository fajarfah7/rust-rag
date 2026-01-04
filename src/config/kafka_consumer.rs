use rdkafka::{
    ClientConfig,
    consumer::{Consumer, StreamConsumer},
};

pub fn create_kafka_consumer(group_id: &str, topics: &[&str]) -> StreamConsumer {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("consumer error");

    consumer.subscribe(topics).expect("subscribe failed");
    consumer
}
// let consumer: StreamConsumer = ClientConfig::new()
//     .set("group.id", "document-parser-consumer")
//     .set("bootstrap.servers", "localhost:9092")
//     .set("enable.auto.commit", "true")
//     .set("auto.offset.reset", "earliest")
//     .create()
//     .expect("consumer error");
// consumer.subscribe(&["document-parser"]).unwrap();
