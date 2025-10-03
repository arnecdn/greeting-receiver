use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use async_trait::async_trait;
use chrono::{NaiveDateTime};
use derive_more::Display;
use tracing::instrument;
use uuid::Uuid;

#[async_trait]
pub trait GreetingService: Sync + Send + Debug {
    async fn receive_greeting(
        &mut self,
        greeting: Greeting,
    ) -> Result<(), ServiceError>;

    async fn check_liveness(
        &mut self,
    ) -> Result<(), ServiceError>;
}

#[async_trait]
pub trait GreetingRepository: Sync + Send {
    async fn store(&mut self, greeting: Greeting) -> Result<(), ServiceError>;
    async fn peek_topic(&mut self) -> Result<(), ServiceError>;
}

pub struct GreetingServiceImpl<C> {
    repo: C,
}

impl<C: GreetingRepository + Sync + Send> GreetingServiceImpl<C> {
    pub fn new(repo: C) -> GreetingServiceImpl<C> {
        GreetingServiceImpl { repo }
    }
}

impl<C: GreetingRepository + Sync + Send> Debug for GreetingServiceImpl<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GreetingRepository(repo)")
    }
}

#[async_trait]
impl<C: GreetingRepository + Sync + Send> GreetingService for GreetingServiceImpl<C> {
    #[instrument]
    async fn receive_greeting(
        &mut self,
        greeting: Greeting,
    ) -> Result<(), ServiceError> {
        self.repo.store(greeting).await
    }

    async fn check_liveness(&mut self) -> Result<(), ServiceError> {
        self.repo.peek_topic().await
    }
}

#[derive(Debug, Display)]
pub enum ServiceError {
    RepoError(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Greeting {
    pub(crate) external_reference: String,
    pub(crate) id: String,
    pub(crate) to: String,
    pub(crate) from: String,
    pub(crate) heading: String,
    pub(crate) message: String,
    pub(crate) created: NaiveDateTime,
    pub(crate) events_created: HashMap<String, NaiveDateTime>,
}

impl Greeting {
    pub fn new(
        greeting_id: String,
        to: String,
        from: String,
        heading: String,
        message: String,
        time: NaiveDateTime,
    ) -> Greeting {
        Greeting {
            external_reference: greeting_id,
            id: String::from(Uuid::now_v7()),
            to,
            from,
            heading,
            message,
            created: time,
            events_created: HashMap::new(),
        }
    }

    pub fn add_event(&mut self, event: &str){
        self.events_created.insert(String::from(event), NaiveDateTime::default());
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn test_receive_greeting() {
        let mock_repo = MockGreetingRepository::new();
        let mut service = GreetingServiceImpl::new(mock_repo);

        let greeting = Greeting::new(
            String::from("test_id"),
            String::from("John"),
            String::from("Jane"),
            String::from("Hello"),
            String::from("Hi John!"),
            NaiveDateTime::default(),
        );

        let result = service.receive_greeting(greeting.clone());
        assert!(block_on(result).is_ok());

    }
    #[test]
    fn check_liveness() {
        let mock_repo = MockGreetingRepository::new();
        let mut service = GreetingServiceImpl::new(mock_repo);

        let result = service.check_liveness();
        assert!(block_on(result).is_ok());

    }

    struct MockGreetingRepository {
        greetings: Vec<Greeting>,
    }

    impl MockGreetingRepository {
        fn new() -> Self {
            Self {
                greetings: Vec::new(),
            }
        }
    }
    #[async_trait]
    impl GreetingRepository for MockGreetingRepository {
        async fn store(
            &mut self,
            greeting: Greeting,
        ) -> Result<(), ServiceError> {
            let repo = &mut self.greetings;
            repo.push(greeting);
            Ok(())
        }

        async fn peek_topic(&mut self) -> Result<(), ServiceError> {
            Ok(())
        }
    }
}
