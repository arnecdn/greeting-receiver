use crate::greeting::service::{Greeting, GreetingRepository, ServiceError};
use crate::settings::Kafka;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::info;
use opentelemetry::global;
use opentelemetry::propagation::Injector;
use rdkafka::error::KafkaError;
use rdkafka::message::{Header, Headers, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use rdkafka::ClientConfig;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::time::Duration;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub struct KafkaGreetingRepository {
    producer: FutureProducer,
    topic: String,
}
impl KafkaGreetingRepository {
    pub fn new(config: Kafka, transactional_id: &str) -> Result<Self, ServiceError> {
        let p: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", config.broker.clone())
            .set("message.timeout.ms", config.message_timeout_ms.to_string())
            .set("debug", "all")
            .set("enable.idempotence", config.enable_idempotence.to_string())
            .set("transactional.id", transactional_id)
            .set("message.send.max.retries", "2")
            .create()
            .map_err(|e| ServiceError::RepoError(format!("Producer creation error: {}", e)))?;

        p.init_transactions(Duration::from_secs(5))
            .map_err(|e| ServiceError::RepoError(format!("Failed to init transactions: {}", e)))?;

        Ok(KafkaGreetingRepository {
            producer: p,
            topic: String::from(config.topic.clone()),
        })
    }
}
#[async_trait]
impl GreetingRepository for KafkaGreetingRepository {
    async fn store(&mut self, greeting: Greeting) -> Result<(), ServiceError> {
        let msg = GreetingMessage::from(&greeting.clone());
        let x = serde_json::to_string(&msg).unwrap();
        let parent_context = Span::current().context();

        let mut headers = OwnedHeaders::new().insert(Header {
            key: "id",
            value: Some(&msg.message_id),
        });

        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&parent_context, &mut HeaderInjector(&mut headers))
        });

        self.producer
            .begin_transaction()
            .map_err(|e| ServiceError::RepoError(format!("Failed beginning transaction: {}", e)))?;
        info!("Sending msg id {}", msg.message_id);

        let kafka_tx_timeout = Duration::from_secs(5);

        let send_result =  self.producer
            .send(
                FutureRecord::to(&self.topic)
                    .headers(headers)
                    .payload(&x)
                    .key(&msg.message_id)
                    .partition(-1),
                kafka_tx_timeout,
            )
            .await;

        if let Err((e, _)) = send_result {
            let _ = self.producer.abort_transaction(kafka_tx_timeout);
            info!("Failed sending message: {}", e);
            return Err(ServiceError::RepoError(format!("Failed sending message: {}", e)));
        }

        if let Err(e) = self.producer.commit_transaction(kafka_tx_timeout) {
            let _ = self.producer.abort_transaction(kafka_tx_timeout);
            info!("Failed sending message: {}", e);
            return Err(ServiceError::RepoError(format!("Error committing transaction: {}", e)));
        }

        Ok(())
    }
}
impl From<KafkaError> for ServiceError {
    fn from(_error: KafkaError) -> Self {
        ServiceError::RepoError(_error.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GreetingMessage {
    external_reference: String,
    message_id: String,
    to: String,
    from: String,
    heading: String,
    message: String,
    created: DateTime<Utc>,
    events_created: HashMap<String, DateTime<Utc>>,
}

impl From<&Greeting> for GreetingMessage {
    fn from(greeting: &Greeting) -> Self {
        GreetingMessage {
            external_reference: greeting.external_reference.to_string(),
            message_id: greeting.message_id.to_string(),
            to: greeting.to.to_string(),
            from: greeting.from.to_string(),
            heading: greeting.heading.to_string(),
            message: greeting.message.to_string(),
            created: *&greeting.created,
            events_created: greeting.events_created.clone(),
        }
    }
}

pub struct HeaderInjector<'a>(pub &'a mut OwnedHeaders);

impl<'a> Injector for HeaderInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        let mut new = OwnedHeaders::new().insert(rdkafka::message::Header {
            key,
            value: Some(&value),
        });

        for header in self.0.iter() {
            let s = String::from_utf8(header.value.unwrap().to_vec()).unwrap();
            new = new.insert(rdkafka::message::Header {
                key: header.key,
                value: Some(&s),
            });
        }

        self.0.clone_from(&new);
    }
}
