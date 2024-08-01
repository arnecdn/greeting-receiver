

use std::process::exit;
use std::sync::RwLock;

use actix_web::{App, HttpServer};

use actix_web::web::Data;
use chrono::Local;
use log::{info, Level, LevelFilter, Metadata, Record};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use greeting::{api, service};
use settings::Settings;

use crate::greeting::service::GreetingService;
use crate::greeting::kafka_producer::KafkaGreetingRepository;

mod greeting;
mod settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        info(description = "Greeting Api description"),
        paths(api::greet, api::list_greetings),
        components(schemas(api::GreetingDto))
    )]

    struct ApiDoc;
    log::set_logger(&CONSOLE_LOGGER).expect("Not able to config logger");
    log::set_max_level(LevelFilter::Info);

    info!("Starting server");
    let app_config = Settings::new();
    let repo = match KafkaGreetingRepository::new(app_config.kafka , "producer_1"){
        Ok(r) => r,
        Err(e) => {
            println!("{:?}", e);
            exit(1)
        }
    };

    //Need explicit type in order to enforce type restrictions with dynamoc trait object allocation
    let service_impl = service::GreetingServiceImpl::new(repo);
    let svc: Data<RwLock<Box<dyn GreetingService + Sync + Send>>> = Data::new(RwLock::new(
        Box::new(service_impl),
    ));



    HttpServer::new(move || {
        App::new()
            .app_data(svc.clone())
            .service(api::greet)
            .service(api::list_greetings)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run().await
}

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}: {} - {}", Local::now(),record.level(), record.args());
        }
    }

    fn flush(&self) {}
}