use chrono::{DateTime, Utc};

// use crate::greeting::repository::{ GreetingRepository};
#[derive(Debug)]
pub enum ServiceError{
    UnrecognizedGreetingError,
}

pub trait GreetingService {
    fn receive_greeting(&mut self,  greeting: Greeting) -> Result<(), ServiceError>;
    fn all_greetings(&self) -> Result<Vec<Greeting>, ServiceError>;

}

pub trait GreetingRepository {
    // Get all greetings
    fn all(&self) -> Result<Vec<Greeting>, ServiceError>;

    // Create a new greeting
    fn store(&mut self, greeting: Greeting) -> Result<(), ServiceError>;
}

struct GreetingServiceImpl{
    repo: Box<dyn GreetingRepository>
}

impl GreetingServiceImpl {
    pub fn new(repo: Box<dyn GreetingRepository>) -> Box<GreetingServiceImpl> {
        Box::new(GreetingServiceImpl {
            repo
        })
    }
}

impl GreetingService for GreetingServiceImpl {
    fn receive_greeting(&mut self, greeting: Greeting) -> Result<(), ServiceError> {
        self.repo.store(greeting)
    }

    fn all_greetings(&self) -> Result<Vec<Greeting>, ServiceError> {
        self.repo.all()
    }
}


#[derive( Clone,PartialEq, Debug)]
pub struct Greeting{
    pub(crate) to: String,
    pub(crate) from: String,
    pub(crate) heading: String,
    pub(crate) message: String,
    pub(crate) created: DateTime<Utc>,
}

impl Greeting {
    pub fn new(to: String, from: String, heading: String, message: String) -> Greeting {
        Greeting {
            to,
            from,
            heading,
            message,
            created: Utc::now(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receive_greeting() {
        // Arrange
        let  mock_repo = MockGreetingRepository::new();
        let mut service = GreetingServiceImpl::new(Box::new(mock_repo));

        let greeting = Greeting::new(String::from("John"), String::from("Jane"), String::from("Hello"), String::from("Hi John!"));

        // Act
        let result = service.receive_greeting(greeting.clone());

        // Assert
        assert!(result.is_ok());
        assert_eq!(service.all_greetings().unwrap(), vec![greeting]);
    }
}

struct MockGreetingRepository {
     greetings: Vec<Greeting>
}

impl MockGreetingRepository {
    fn new() -> Self {
        Self {
            greetings: Vec::new()
        }
    }
}

impl GreetingRepository for MockGreetingRepository {
    fn all(&self) -> Result<Vec<Greeting>, ServiceError> {
        Ok(self.greetings.clone())
    }

    fn store(&mut self, greeting: Greeting) -> Result<(), ServiceError> {
        let repo = &mut self.greetings;
        repo.push(greeting);
        Ok(())
    }
}


