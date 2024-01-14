use std::collections::HashMap;
use std::sync::RwLock;

use actix_web::{get, HttpResponse, put, ResponseError, web};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use chrono::{DateTime, Utc};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};
use validator_derive::Validate;

use crate::greeting::api::ApiError::BadClientData;

#[utoipa::path(
    get,
    path = "/greeting",
    responses(
        (status = 200, description = "Greetings", body = GreetingDto),
        (status = NOT_FOUND, description = "Greetings was not found")
    )
    )]
#[get("/greeting")]
pub async fn list_greetings(
    data: web::Data<RwLock<HashMap<usize, GreetingDto>>>,
) -> Result<HttpResponse, ApiError> {
    let guarded_data = data.read().unwrap();
    let v = guarded_data.values().collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(v))
}

#[utoipa::path(
    put,
    path = "/greeting",
    responses(
        (status = 201, description = "Greeting successfully stored", body = GreetingDto),
        (status = NOT_FOUND, description = "Resource not found")
    ),
    )]
#[put("/greeting")]
pub async fn greet(
    data: web::Data<RwLock<HashMap<usize, GreetingDto>>>,
    greeting: web::Json<GreetingDto>,
) -> Result<HttpResponse, ApiError> {
    greeting.validate()?;

    let mut guarded_data = data.write().unwrap();
    let key = &guarded_data.len() + 1;
    guarded_data.insert(key, greeting.0);
    Ok(HttpResponse::Ok().body(""))
}

#[derive(Debug, Display, Error)]
pub enum ApiError {
    BadClientData(ValidationErrors),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            BadClientData(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(value: ValidationErrors) -> Self {
        BadClientData(value)
    }
}

#[derive(Validate, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GreetingDto {
    #[validate(length(min = 1, max = 20))]
    to: String,
    #[validate(length(min = 1, max = 20))]
    from: String,
    #[validate(length(min = 1, max = 50))]
    heading: String,
    #[validate(length(min = 1, max = 50))]
    message: String,
    #[schema(value_type = String, format = DateTime)]
    created: DateTime<Utc>,
}


impl GreetingDto {
    // pub fn new(to: &str, from: &str, heading: &str, message: &str) -> Self {
    //     Self {
    //         to: to.to_string(),
    //         from: from.to_string(),
    //         heading: heading.to_string(),
    //         message: message.to_string(),
    //         created: Utc::now(),
    //     }
    // }
    //
    // pub fn to(&self) -> Greeting {
    //     Greeting {
    //
    //         to: self.to.clone(),
    //         from: self.from.clone(),
    //         heading: self.heading.clone(),
    //         message: self.message.clone(),
    //         created: self.created,
    //     }
    // }
    //
    // pub fn from(greeting: &Greeting) -> Self {
    //     Self {
    //         to: greeting.to.clone(),
    //         from: greeting.from.clone(),
    //         heading: greeting.heading.clone(),
    //         message: greeting.message.clone(),
    //         created: greeting.created,
    //     }
    // }
}

#[cfg(test)]
mod test {
    use actix_web::test;

    use super::*;

    #[actix_web::test]
    async fn test_read_greeting() {
        let  repo = HashMap::new();

        let app = test::init_service(crate::test_app!(repo, list_greetings)).await;

        let req = test::TestRequest::get()
            .uri("/greeting")
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_store_greeting() {
        let app = test::init_service(crate::test_app!(HashMap::new(), greet)).await;

        let req = test::TestRequest::put()
            .uri("/greeting")
            .insert_header(actix_web::http::header::ContentType::json())
            .set_json(GreetingDto {
                to: String::from("test"),
                from: String::from("testa"),
                heading: String::from("Merry Christmas"),
                message: String::from("Happy new year"),
                created: DateTime::default(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_invalid_greeting() {
        let app = test::init_service(crate::test_app!(HashMap::new(), greet)).await;

        let req = test::TestRequest::put()
            .uri("/greeting")
            .insert_header(actix_web::http::header::ContentType::json())
            .set_json(GreetingDto {
                to: String::from("testtesttesttesttesttesttesttest"),
                from: String::from("testa"),
                heading: String::from("Merry Christmas"),
                message: String::from("Happy new year"),
                created: DateTime::default(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
        println!("{:?}", resp.response().body());
    }
}

#[macro_export]
macro_rules! test_app {
    ($data:expr,$service:expr) => {{
        let greeting_store: actix_web::web::Data<RwLock<HashMap<usize, GreetingDto>>> =
            actix_web::web::Data::new(RwLock::new($data));

        actix_web::App::new()
            .app_data(greeting_store.clone())
            .service($service)
    }};
}
