use std::collections::HashMap;

use std::sync::{Arc, RwLock};

use actix_web::{App,  HttpServer};
use actix_web::web::Data;
// use serde::{Deserialize, Deserializer, Serialize};
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use greeting::{api,repository, service};

mod greeting;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
    info(description = "Greeting Api description"),
    paths(api::greet, api::list_greetings),
    components(schemas(api::GreetingDto))
    )]
    struct ApiDoc;

    let data = HashMap::new();
    let repo = repository::GreetingRepositoryInMemory::new(data);
    let svc = Data::new(RwLock::new(service::GreetingServiceImpl::new(repo)));

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

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
