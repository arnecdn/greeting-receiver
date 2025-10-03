use std::process::exit;
use std::sync::RwLock;

use actix_web::{App, HttpServer};

use actix_web::web::Data;
use log::{error, info};

use greeting::{api, service};
use settings::Settings;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::greeting::kafka_producer::KafkaGreetingRepository;
use crate::greeting::service::GreetingService;

use actix_web_opentelemetry::RequestMetrics;
use greeting_otel::init_otel;

mod greeting;
mod settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        info(description = "Greeting Api description"),
        paths(api::greet, api::health),
        components(schemas(api::GreetingDto))
    )]

    struct ApiDoc;

    let app_config = Settings::new();

    // Initialize logs and save the logger_provider.
    let providers = init_otel(
        &app_config.otel_collector.oltp_endpoint,
        "greeting_receiver",
        &app_config.kube.my_pod_name,
    )
    .await;

    info!("Starting server");
    let transaction_id = format!("producer_1_{}", &app_config.kube.my_pod_name.clone());
    let repo = match KafkaGreetingRepository::new(app_config.kafka, &transaction_id) {
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
            .wrap(RequestMetrics::default())
            .app_data(svc.clone())
            .service(api::greet)
            .service(api::health)
            //.wrap(RequestTracing::default())
            //.wrap(RequestMetrics::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
    .expect("Error stopping server");

    if let Err(e) = providers.shutdown().await{
        error!("Failed to shut down: {:?}", e);
    }
    Ok(())
}
