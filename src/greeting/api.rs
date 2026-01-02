use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse, ResponseError};

use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::{RwLock};
use tracing::instrument;

use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};
use validator_derive::Validate;

use crate::greeting::api::ApiError::{ApplicationError, BadClientData, UnknownError};
use crate::greeting::service::{Greeting, GreetingService, ServiceError};

#[utoipa::path(
    post,
    path = "/greeting",
    responses(
        (status = 201, description = "Greeting successfully stored", body = GreetingDto),
        (status = NOT_FOUND, description = "Resource not found")
    ),
    )]
#[post("/greeting")]
#[instrument(name = "receive")]
pub async fn greet(
    data: Data<RwLock<Box<dyn GreetingService + Sync + Send>>>,
    greeting: web::Json<GreetingDto>,
) -> Result<HttpResponse, ApiError> {
    greeting.validate()?;
    match data.write() {
        Ok(mut guard) => {
            info!("Received greeting {}", &greeting.0.heading);
            let greeting_msg:Greeting = greeting.0.into();
            let resp = GreetingReceived {message_id: greeting_msg.message_id.clone()};

            guard.receive_greeting(greeting_msg).await?;
            Ok(HttpResponse::Ok().json(resp))
        }

        Err(e) => Err(UnknownError(format!(
            "Failed writing  msg:{:?} to kafka with error: {}",
            greeting,
            e.to_string()
        ))),
    }
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health is responding"),
        (status = NOT_FOUND, description = "Resource not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Resource failed")
    ),
)]
#[get("/health")]
#[instrument(name = "health")]
pub async fn health(
    data: Data<RwLock<Box<dyn GreetingService + Sync + Send>>>,
) -> Result<HttpResponse, ApiError> {
    match data.write() {
        Ok(mut guard) => {
            guard.check_liveness().await?;
            return Ok(HttpResponse::Ok().body(""));
        }

        Err(e) => Err(UnknownError(format!(
            "Failed checking health with error: {}",
            e.to_string()
        ))),
    }
}

#[derive(Debug)]
pub enum ApiError {
    BadClientData(ValidationErrors),
    ApplicationError(ServiceError),
    UnknownError(String),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            BadClientData(_) => StatusCode::BAD_REQUEST,
            ApplicationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadClientData(e) => write!(f, "Bad client data: {}", e),
            ApplicationError(e) => write!(f, "Application error: {}", e),
            UnknownError(msg) => write!(f, "Application error: {}", msg),
        }
    }
}
impl From<ValidationErrors> for ApiError {
    fn from(value: ValidationErrors) -> Self {
        BadClientData(value)
    }
}

impl From<ServiceError> for ApiError {
    fn from(value: ServiceError) -> Self {
        ApplicationError(value)
    }
}

#[derive(Validate, Serialize, Deserialize, Clone, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GreetingDto {
    external_reference: String,
    #[validate(length(min = 1, max = 36))]
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

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GreetingReceived {
    message_id: String,
}

impl Into<Greeting> for GreetingDto {
    fn into(self) -> Greeting {
        let mut received_greeting = Greeting::new(
            self.external_reference.clone(),
            self.to.clone(),
            self.from.clone(),
            self.heading.clone(),
            self.message.clone(),
            self.created.naive_utc(),
        );
        received_greeting.add_event(&"received_greeting");
        received_greeting
    }
}
impl From<Greeting> for GreetingDto {
    fn from(greeting: Greeting) -> Self {
        GreetingDto {
            external_reference: greeting.external_reference.clone(),
            to: greeting.to.clone(),
            from: greeting.from.clone(),
            heading: greeting.heading.clone(),
            message: greeting.message.clone(),
            created: greeting.created.and_utc(),
        }
    }
}

#[cfg(test)]
mod test {
    use actix_web::test;
    use async_trait::async_trait;
    use super::*;

    #[actix_web::test]
    async fn test_store_greeting() {
        let data: Data<RwLock<Box<dyn GreetingService + Sync + Send>>> =
            Data::new(RwLock::new(Box::new(GreetingSvcStub {})));
        let app =
            test::init_service(actix_web::App::new().app_data(data.clone()).service(greet)).await;

        let req = test::TestRequest::post()
            .uri("/greeting")
            .insert_header(actix_web::http::header::ContentType::json())
            .set_json(GreetingDto {
                external_reference: "1".to_string(),
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
        let data: Data<RwLock<Box<dyn GreetingService + Sync + Send>>> =
            Data::new(RwLock::new(Box::new(GreetingSvcStub {})));
        let app =
            test::init_service(actix_web::App::new().app_data(data.clone()).service(greet)).await;

        let req = test::TestRequest::post()
            .uri("/greeting")
            .insert_header(actix_web::http::header::ContentType::json())
            .set_json(GreetingDto {
                external_reference: "1".to_string(),
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

    #[actix_web::test]
    async fn test_health_liveness() {
        let data: Data<RwLock<Box<dyn GreetingService + Sync + Send>>> =
            Data::new(RwLock::new(Box::new(GreetingSvcStub {})));
        let app =
            test::init_service(actix_web::App::new().app_data(data.clone()).service(greet)).await;

        let req = test::TestRequest::get().uri("/health").to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
        println!("{:?}", resp.response().body());
    }


    #[derive(Clone, Debug)]
    pub struct GreetingSvcStub;

    #[async_trait]
    impl GreetingService for GreetingSvcStub {
        async fn receive_greeting(&mut self, _: Greeting) -> Result<(), ServiceError> {
            Ok(())
        }

        async fn check_liveness(&mut self) -> Result<(), ServiceError> {
            Ok(())
        }
    }
}
