use std::process::exit;
use std::sync::RwLock;

use actix_web::{App, HttpServer};

use actix_web::web::Data;

use log::{error, info, Level};
use opentelemetry::{global, KeyValue};
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::logs::LoggerProvider;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_stdout::SpanExporter;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use greeting::{api, service};
use settings::Settings;

use crate::greeting::kafka_producer::KafkaGreetingRepository;
use crate::greeting::service::GreetingService;

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
    init_logging();
    init_tracer();

    info!("Starting server");
    let app_config = Settings::new();
    let repo = match KafkaGreetingRepository::new(app_config.kafka, "producer_1") {
        Ok(r) => r,
        Err(e) => {
            error!("{:?}", e);
            exit(1)
        }
    };

    //Need explicit type in order to enforce type restrictions with dynamoc trait object allocation
    let service_impl = service::GreetingServiceImpl::new(repo);
    let svc: Data<RwLock<Box<dyn GreetingService + Sync + Send>>> =
        Data::new(RwLock::new(Box::new(service_impl)));

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
    .run()
    .await
}

fn init_tracer() {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let provider = TracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build();
    global::set_tracer_provider(provider);

}

fn init_logging() {
    // Setup LoggerProvider with a stdout exporter
    let exporter = opentelemetry_stdout::LogExporterBuilder::default()
        // uncomment the below lines to pretty print output.
        // .with_encoder(|writer, data|
        //    Ok(serde_json::to_writer_pretty(writer, &data).unwrap()))
        .build();
    let logger_provider = LoggerProvider::builder()
        .with_resource(Resource::new([KeyValue::new(
            SERVICE_NAME,
            "greeting_rust",
        )]))
        .with_simple_exporter(exporter)
        .build();

    // Setup Log Appender for the log crate.
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
    log::set_max_level(Level::Info.to_level_filter());
}