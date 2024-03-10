use log::{info, warn};
use rdkafka::{ClientConfig, ClientContext, Message, TopicPartitionList};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use serde::{Deserialize, Serialize};

struct CustomContext{
    id: String
}

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

pub(crate) async fn consume_and_print(consumer_id: String, brokers: String, group_id: String, topics: String) {

    let context = CustomContext{id:String::from(&consumer_id) };

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", &brokers)
         .set("enable.auto.commit", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&topics])
        .expect("Can't subscribe to specified topics");

    info!(
        "Consumer: {} is listening on topic:{:?} on broker: {:?}",
        &consumer_id,
        &[&topics],
        &brokers
    );
    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(msg) => {
                let key: &str = msg.key_view().unwrap().unwrap();
                let payload: KafkaTopicPayload = serde_json::from_slice(msg.payload().unwrap())
                    .expect("failed to deser JSON to GreetingLogg");
                info!(
                    "Consumer {} received key {} with value {:?} in offset {:?} from partition {} ",
                    &consumer_id,
                    key,
                    payload.after,
                    msg.offset(),
                    msg.partition()
                );
                consumer.commit_message(&msg, CommitMode::Async).unwrap();
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KafkaTopicPayload {
    after: GreetingLogg,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde_with::serde_as]
pub struct GreetingLogg {
    id: i64,
    greeting_id: String,
    // #[serde_as(as = "TimestampSeconds<i64>")]
    created: i64,
}
