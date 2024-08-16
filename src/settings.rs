use config::Config;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub (crate) struct Settings {
    pub (crate) kafka: Kafka,
    pub (crate) otel_collector: OtelCollector

}

impl Settings {
    pub fn new() -> Self {

        dotenv().ok();

        let settings = Config::builder()
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
}

#[derive(Deserialize)]
pub (crate) struct OtelCollector{
    pub (crate) oltp_endpoint: String
}
