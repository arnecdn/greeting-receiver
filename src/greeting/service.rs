use async_trait::async_trait;
use chrono::NaiveDateTime;
use derive_more::{Display, Error, FromStr};
use crate::greeting::repository::RepoError;


#[async_trait]
pub trait GreetingService: Sync + Send  {
     async fn receive_greeting(&mut self,  greeting: Greeting) -> Result<(), ServiceError>;
    async fn all_greetings(&self) -> Result<Vec<Greeting>, ServiceError>;

}

#[async_trait]
pub trait GreetingRepository: Sync + Send  {
    async fn all(&self) -> Result<Vec<Greeting>, ServiceError>;

    async fn store(&mut self, greeting: Greeting) -> Result<(), ServiceError>;
}



pub struct GreetingServiceImpl<C>{
    repo: C
}

impl <C:GreetingRepository+Sync + Send > GreetingServiceImpl<C> {
    pub fn new(repo: C) -> GreetingServiceImpl<C> {
        GreetingServiceImpl {
            repo
        }
    }
}

#[async_trait]
impl <C:GreetingRepository+  Sync + Send > GreetingService for GreetingServiceImpl<C>  {
    async fn receive_greeting(&mut self, greeting: Greeting) -> Result<(), ServiceError> {
        self.repo.store(greeting).await
    }

    async fn all_greetings(&self) -> Result<Vec<Greeting>, ServiceError> {
        self.repo.all().await
    }
}

#[derive(Debug, Display)]
pub enum ServiceError{
    UnrecognizedGreetingError,
    RepoError(String)
}

#[derive( Clone,PartialEq, Debug)]
pub struct Greeting{

    pub(crate) to: String,
    pub(crate) from: String,
    pub(crate) heading: String,
    pub(crate) message: String,
    pub(crate) created: NaiveDateTime,
}

impl Greeting {
    pub fn new(to: String, from: String, heading: String, message: String) -> Greeting {
        Greeting {
            to,
            from,
            heading,
            message,
            created: NaiveDateTime::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
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
        assert!(block_on(result).is_ok());
        let all_result = service.all_greetings();

        assert_eq!(block_on(all_result).unwrap(), vec![greeting]);
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
    #[async_trait]
    impl GreetingRepository for MockGreetingRepository {
        async fn all(&self) -> Result<Vec<Greeting>, ServiceError> {
            Ok(self.greetings.clone())
        }

        async fn store(&mut self, greeting: Greeting) -> Result<(), ServiceError> {
            let  repo = &mut self.greetings;
            repo.push(greeting);
            Ok(())
        }
    }

}


