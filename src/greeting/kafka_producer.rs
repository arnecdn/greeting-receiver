use std::time::Duration;
use async_trait::async_trait;

use chrono::{DateTime, NaiveDateTime, Utc};
use futures::executor::block_on;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use rdkafka::ClientConfig;
use rdkafka::error::KafkaError;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

use crate::greeting::service::{Greeting, GreetingRepository, ServiceError};

pub struct KafkaGreetingRepository {
    // producer: FutureProducer,
    brokers: String,
    topic: String,
    transactional_producer: String
}
impl KafkaGreetingRepository{
    pub fn new(brokers: &str, topic: &str, transactional_producer: &str) -> Result<Self, ServiceError>{


        Ok(KafkaGreetingRepository {brokers: String::from(brokers),topic: String::from(topic), transactional_producer: String::from(transactional_producer) })
    }
}
#[async_trait]
impl GreetingRepository for KafkaGreetingRepository {
    async fn all(&self) -> Result<Vec<Greeting>, ServiceError> {
        panic!("Not implementd")
    }

    async fn store(&mut self, greeting: Greeting) -> Result<(), ServiceError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &self.brokers)
            .set("message.timeout.ms", "5000")
            // .set("enable.idempotence", "true")
            // .set("transactional.id", &self.transactional_producer)
            .create()
            .expect("Producer creation error with invalid configuration");

        // producer
        //     .init_transactions(Duration::from_secs(5))
        //     .expect("Expected to init transactions");

        let msg = GreetingMessage::from(&greeting.clone());
        let x = serde_json::to_string(&msg).unwrap();
        // producer
        //     .begin_transaction()
        //     .expect("Failed beginning transaction");

        producer.send(
            FutureRecord::to(&self.topic).payload(&x).key(&msg.id),
            Duration::from_secs(0),
        ).await;
        Ok(())
        //
        // Ok(producer
        //     .commit_transaction(Duration::from_secs(5))?)


    }
}
impl From<KafkaError> for ServiceError {
    fn from(_error: KafkaError) -> Self {
        ServiceError::RepoError(_error.to_string())
    }
}
// pub(crate) fn produce(brokers: &str, topic_name: &str, greeting: GreetingEntity) {
    //
    // let producer: &FutureProducer = &ClientConfig::new()
    //     .set("bootstrap.servers", brokers)
    //     .set("message.timeout.ms", "5000")
    //     .set("enable.idempotence", "true")
    //     .set("transactional.id ", "message_id")
    //     .create()
    //     .expect("Producer creation error with invalid configuration");
    //
    // producer
    //     .init_transactions(Duration::from_secs(5))
    //     .expect("Expected to init transactions");

    // let msg = GreetingMessage::from(&greeting.clone());
    // let x = serde_json::to_string(&msg).unwrap();
    // producer
    //     .begin_transaction()
    //     .expect("Failed beginning transaction");
    //
    // let future = producer.send(
    //     FutureRecord::to(topic_name).payload(&x).key(&msg.id),
    //     Duration::from_secs(0),
    // );
    // producer
    //     .commit_transaction(Duration::from_secs(5))
    //     .expect("Unable to commit transaction`");
    // block_on(future).expect("received");
// }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GreetingMessage {
    id: String,
    to: String,
    from: String,
    heading: String,
    message: String,
    created: NaiveDateTime,
}

impl From<&Greeting> for GreetingMessage {
    fn from(greeting: &Greeting) -> Self {
        GreetingMessage {
            id: String::from(Uuid::now_v7()),
            to: greeting.to.to_string(),
            from: greeting.from.to_string(),
            heading: greeting.heading.to_string(),
            message: greeting.message.to_string(),
            created: *&greeting.created,
        }
    }
}
