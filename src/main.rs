use actix_web::{App, HttpServer};

use actix_web::web::Data;
use log::{error, info};

use greeting::api;
use settings::Settings;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
        components(schemas(api::GreetingDto, api::GreetingReceived))
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

    let kafka_config = Data::new(app_config.kafka);

    HttpServer::new(move || {
        App::new()
            .wrap(RequestMetrics::default())
            .app_data(kafka_config.clone())
            .service(api::greet)
            .service(api::health)
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
