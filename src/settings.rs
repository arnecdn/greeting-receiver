use config::Config;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub (crate) struct Settings {
    pub (crate) kafka: Kafka
}

impl Settings {
    pub fn new() -> Self {

        dotenv().ok();
        // let env = env::var("environment").unwrap_or("development".into());

        let settings = Config::builder()
            .add_source(config::File::with_name("./res/server").required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()
            .unwrap();

        settings.try_deserialize().unwrap()

    }
}
#[derive(Deserialize)]
pub (crate) struct Kafka {
    pub (crate) broker: String,
    pub (crate) topic: String,
    pub (crate) message_timeout_ms: i32,
    pub (crate) enable_idempotence: bool,
    pub (crate) processing_guarantee: String,
}
