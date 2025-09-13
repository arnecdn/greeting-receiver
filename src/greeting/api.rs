use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{ post, web, HttpResponse, ResponseError};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_more::Display;
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;
use tracing::{instrument};

use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};
use validator_derive::Validate;

use crate::greeting::api::ApiError::{ApplicationError, Applicationerror, BadClientData};
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

    if let Ok(mut guard) = data.write() {
        info!("Received greeting {}", &greeting.0.heading);
        guard.receive_greeting(greeting.0.into()).await?;
        return Ok(HttpResponse::Ok().body(""));
    }
    Err(Applicationerror)
}

#[derive(Debug, Display)]
pub enum ApiError {
    BadClientData(ValidationErrors),
    ApplicationError(ServiceError),
    Applicationerror,
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

impl Into<Greeting> for GreetingDto {
    fn into(self) -> Greeting {
        Greeting::new(
            self.to.clone(),
            self.from.clone(),
            self.heading.clone(),
            self.message.clone(),
            self.created.naive_utc(),
        )
    }
}
impl From<Greeting> for GreetingDto {
    fn from(greeting: Greeting) -> Self {
        GreetingDto {
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
#[derive(Clone, Debug)]
pub struct GreetingSvcStub;

#[async_trait]
impl GreetingService for GreetingSvcStub {
    async fn receive_greeting(&mut self, _: Greeting) -> Result<(), ServiceError> {
        Ok(())
    }
}
