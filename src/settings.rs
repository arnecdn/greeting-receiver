use config::Config;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GreetingsAppConfig {
    pub kafka_consumer: KafkaConfig,
    pub database: Database
}

impl GreetingsAppConfig {
    pub fn new() -> Self {

        dotenv().ok();
        // let env = env::var("environment").unwrap_or("development".into());

        let settings = Config::builder()
            .add_source(config::File::with_name("./res/server").required(false))
            .add_source(config::Environment::with_prefix("APP").separator("_"))
            .build()
            .unwrap();

        settings.try_deserialize().unwrap()

    }
}
#[derive(Deserialize)]
pub (crate) struct KafkaConfig {
    pub broker: String,
    pub topic: String,
    pub consumer_group: String,
    pub message_timeout_ms: i32,
    pub enable_idempotence: bool,
    pub processing_guarantee: String,
    pub number_of_consumers:i32
}
#[derive(Deserialize)]
pub (crate) struct Database {
    pub(crate) url: String,
    // user: String,
    // password: String,
    // host: String,
    // database: String
}