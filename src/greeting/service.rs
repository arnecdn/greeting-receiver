use std::fmt::{Debug, Formatter};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use derive_more::Display;
use tracing::instrument;
use uuid::Uuid;

#[async_trait]
pub trait GreetingService: Sync + Send + Debug {
    async fn receive_greeting(
        &mut self,
        greeting: Greeting,
    ) -> Result<(), ServiceError>;
    async fn all_greetings(&self) -> Result<Vec<Greeting>, ServiceError>;
}

#[async_trait]
pub trait GreetingRepository: Sync + Send {
    async fn all(&self) -> Result<Vec<Greeting>, ServiceError>;

    async fn store(&mut self, greeting: Greeting) -> Result<(), ServiceError>;
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

    async fn all_greetings(&self) -> Result<Vec<Greeting>, ServiceError> {
        self.repo.all().await
    }
}

#[derive(Debug, Display)]
pub enum ServiceError {
    RepoError(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Greeting {
    pub(crate) id: String,
    pub(crate) to: String,
    pub(crate) from: String,
    pub(crate) heading: String,
    pub(crate) message: String,
    pub(crate) created: NaiveDateTime,
}

impl Greeting {
    pub fn new(
        to: String,
        from: String,
        heading: String,
        message: String,
        time: NaiveDateTime,
    ) -> Greeting {
        Greeting {
            id: String::from(Uuid::now_v7()),
            to,
            from,
            heading,
            message,
            created: time,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use opentelemetry::trace::TraceContextExt;
    use uuid::Uuid;

    #[test]
    fn test_receive_greeting() {
        let mock_repo = MockGreetingRepository::new();
        let mut service = GreetingServiceImpl::new(mock_repo);

        let greeting = Greeting::new(
            String::from("John"),
            String::from("Jane"),
            String::from("Hello"),
            String::from("Hi John!"),
            NaiveDateTime::default(),
        );

        let result = service.receive_greeting(greeting.clone(), Context::new());
        assert!(block_on(result).is_ok());

        let all_result = service.all_greetings();
        assert_eq!(block_on(all_result).unwrap(), vec![greeting]);
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
        async fn all(&self) -> Result<Vec<Greeting>, ServiceError> {
            Ok(self.greetings.clone())
        }

        async fn store(
            &mut self,
            greeting: Greeting,
        ) -> Result<(), ServiceError> {
            let repo = &mut self.greetings;
            repo.push(greeting);
            Ok(())
        }
    }
}
