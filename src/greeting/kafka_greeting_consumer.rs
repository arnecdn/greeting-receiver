use log::info;
use rdkafka::{ClientConfig, ClientContext, Message, TopicPartitionList};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{
    BaseConsumer, Consumer, ConsumerContext, StreamConsumer,
};
use rdkafka::error::KafkaResult;
use rdkafka::message::Headers;
use serde::{Deserialize, Serialize};

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    // fn pre_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
    //     info!("Pre rebalance {:?}", rebalance);
    // }
    //
    // fn post_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
    //     info!("Post rebalance {:?}", rebalance);
    // }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

pub(crate) async fn consume_and_print(brokers: &str, group_id: &str, topics: &str) {
    let context = CustomContext;

    let consumer: BaseConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        // .set("enable.partition.eof", "false")
        // .set("session.timeout.ms", "6000")
        // .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        // .create_with_context(context)
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&topics])
        .expect("Can't subscribe to specified topics");

    loop {
        info!("Listening on kafka");
        for msg_result in consumer.iter() {
            let msg = msg_result.unwrap();

            let key: &str = msg.key_view().unwrap().unwrap();
            let value = msg.payload_view::<str>().unwrap().unwrap();
            let payload: KafkaTopicPayload = serde_json::from_slice(msg.payload().unwrap())
                .expect("failed to deser JSON to GreetingLogg");
            info!(
                "received key {} with value {:?} in offset {:?} from partition {}",
                key,
                payload.after,
                msg.offset(),
                msg.partition()
            )
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KafkaTopicPayload {
    after: GreetingLogg,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GreetingLogg {
    id: i64,
    greeting_id: String,
    created: i128,
}
