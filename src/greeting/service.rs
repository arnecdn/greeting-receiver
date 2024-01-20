use chrono::{DateTime, Utc};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServiceError{
    UnrecognizedGreetingError,
}

pub trait GreetingService {
    fn receive_greeting(&mut self,  greeting: Greeting) -> Result<(), ServiceError>;
    fn all_greetings(&self) -> Result<Vec<Greeting>, ServiceError>;

}

pub trait GreetingRepository {
    fn all(&self) -> Result<Vec<Greeting>, ServiceError>;

    fn store(&mut self, greeting: Greeting) -> Result<(), ServiceError>;
}

pub struct GreetingServiceImpl<C>{
    repo: C
}

impl <C:GreetingRepository> GreetingServiceImpl<C> {
    pub fn new(repo: C) -> GreetingServiceImpl<C> {
        GreetingServiceImpl {
            repo
        }
    }
}

impl <C:GreetingRepository> GreetingService for GreetingServiceImpl<C>  {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receive_greeting() {
        // Arrange
        let  mock_repo = MockGreetingRepository::new();
        let mut service = GreetingServiceImpl::new(mock_repo);

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


impl GreetingRepository for MockGreetingRepository {
    fn all(&self) -> Result<Vec<Greeting>, ServiceError> {
        Ok(self.greetings.clone())
    }

    fn store(&mut self, greeting: Greeting) -> Result<(), ServiceError> {
        let  repo = &mut self.greetings;
        repo.push(greeting);
        Ok(())
    }
}


