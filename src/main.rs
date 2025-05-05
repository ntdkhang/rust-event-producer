use std::time::Duration;
use chrono::Utc;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use serde_json::{json, Value};
// use rdkafka::message::{OwnedHeaders, Header};


async fn produce(brokers: &str, topic: &str, message: &Value) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let delivery_status = producer
        .send(
            FutureRecord::to(topic)
            .payload(&message.to_string())
            .key(&format!("Key: {:?}", Utc::now())),
            Duration::from_secs(0),
        )
        .await;

    println!("{{ now: {:?}}}", Utc::now());
    println!("Result: {:?}", delivery_status);
}

#[tokio::main]
async fn main() {
    loop {
        let message = json!({
            "user_id": "69420",
            "event": "Nothing happened",
            "timestamp": &format!("{:?}", Utc::now())
        });
        produce("localhost:9094", "quickstart-events", &message)
            .await;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
