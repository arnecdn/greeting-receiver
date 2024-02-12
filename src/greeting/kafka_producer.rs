use std::time::Duration;

use chrono::{DateTime, NaiveDateTime, Utc};
use futures::executor::block_on;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use rdkafka::ClientConfig;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::greeting::repository::GreetingEntity;

pub(crate) fn produce(brokers: &str, topic_name: &str, greeting: GreetingEntity) {

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .set("enable.idempotence", "true")
        .set("transactional.id ", "message_id")
        .create()
        .expect("Producer creation error with invalid configuration");

    producer
        .init_transactions(Duration::from_secs(5))
        .expect("Expected to init transactions");

    let msg = GreetingMessage::from(&greeting.clone());
    let x = serde_json::to_string(&msg).unwrap();
    producer
        .begin_transaction()
        .expect("Failed beginning transaction");

    let future = producer.send(
        FutureRecord::to(topic_name).payload(&x).key(&msg.id),
        Duration::from_secs(0),
    );
    producer
        .commit_transaction(Duration::from_secs(5))
        .expect("Unable to commit transaction`");
    block_on(future).expect("received");
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GreetingMessage {
    id: String,
    to: String,
    from: String,
    heading: String,
    message: String,
    created: NaiveDateTime,
}

impl From<&GreetingEntity> for GreetingMessage {
    fn from(greeting: &GreetingEntity) -> Self {
        GreetingMessage {
            id: String::from(greeting.id),
            to: greeting.to.to_string(),
            from: greeting.from.to_string(),
            heading: greeting.heading.to_string(),
            message: greeting.message.to_string(),
            created: *&greeting.created,
        }
    }
}
