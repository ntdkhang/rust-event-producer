use std::time::Duration;

use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use rdkafka::message::{OwnedHeaders, Header};


async fn produce(brokers: &str, topic: &str) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let futures = (0..5)
        .map(|i| async move {
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic)
                    .payload(&format!("Message {}", i))
                    .key(&format!("Key {}", i))
                    .headers(OwnedHeaders::new().insert(Header {
                        key: "header_key",
                        value: Some("header_value"),
                    })),

                    Duration::from_secs(3),
                )
                .await;

            println!("Delivery status for message {} received", i);
            delivery_status
        })
    .collect::<Vec<_>>();

    for future in futures {
        println!("Result: {:?}", future.await)
    }

}

#[tokio::main]
async fn main() {
    produce("localhost:9094", "quickstart-events")
        .await;
}
