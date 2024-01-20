use std::collections::HashMap;

use std::sync::{RwLock};

use actix_web::{App,  HttpServer};
use actix_web::web::Data;
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use greeting::{api,service};
use crate::greeting::repository::GreetingRepositoryInMemory;
use crate::greeting::service::{ GreetingService};

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
    //Need explicit type in order to enforce type restrictions with dynamoc trait object allocation
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
