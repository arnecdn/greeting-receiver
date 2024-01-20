use std::collections::HashMap;

use std::sync::{Arc, RwLock};

use actix_web::{App,  HttpServer};
use actix_web::web::Data;
// use serde::{Deserialize, Deserializer, Serialize};
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use greeting::{api,repository, service};
use crate::greeting::repository::GreetingRepositoryInMemory;
use crate::greeting::service::{GreetingRepository, GreetingService, GreetingServiceImpl};

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

    let repo = GreetingRepositoryInMemory::new(HashMap::new());
    let svc:  Data<RwLock<Box<dyn GreetingService+ Sync + Send >>>  = Data::new(RwLock::new(Box::new(service::GreetingServiceImpl::new(repo))));

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
