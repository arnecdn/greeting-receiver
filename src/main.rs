use std::collections::HashMap;

use std::sync::RwLock;

use actix_web::{App,  HttpServer};
use actix_web::web::Data;
// use serde::{Deserialize, Deserializer, Serialize};
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use greeting::api;

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
    let greeting_store: Data<RwLock<HashMap<usize, api::GreetingDto>>> =
        Data::new(RwLock::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(greeting_store.clone())
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
