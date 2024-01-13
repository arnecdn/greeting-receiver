use std::error::Error;
use chrono::{DateTime, Utc};
use uuid::Uuid;

enum ServiceError{
    UnrecognizedGreetingError,
}

trait GreetingService {
    fn receive_greeting(&self, greeting: Greeting) -> Result<Greeting, ServiceError>;
    fn read_greeting(&self, id: Uuid) -> Result<Greeting, ServiceError>;

    fn all_greetings(&self) -> Result<Vec<Greeting>, ServiceError>;

}



#[derive( Clone)]
pub struct Greeting{
    id: Uuid,
    to: String,
    from: String,
    heading: String,
    message: String,
    created: DateTime<Utc>,

}