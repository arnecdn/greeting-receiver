use std::{env, fs};
use std::future::Future;
use std::process::exit;
use std::sync::RwLock;

use actix_web::{App, HttpServer};

use actix_web::web::Data;
use serde::Deserialize;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use greeting::{api, service, kafka_greeting_consumer};
use settings::GreetingsAppConfig;

use crate::greeting::repository::SqliteGreetingRepository;
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

    println!("Starting server");
    let app_config = GreetingsAppConfig::new();

    let repo = match SqliteGreetingRepository::new(&app_config.database.url).await {
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

    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    for i in 0..app_config.kafka_consumer.number_of_consumers{
        let kafka_consumer = kafka_greeting_consumer::consume_and_print(String::from("consumer_" ) + &*i.to_string(),
                                                                        app_config.kafka_consumer.broker.clone(),
                                                                        app_config.kafka_consumer.consumer_group.clone(),
                                                                        app_config.kafka_consumer.topic.clone());

        actix_web::rt::spawn(async {  kafka_consumer.await});
    }

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

