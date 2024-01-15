use std::collections::HashMap;
use std::sync::RwLock;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::greeting::service::{Greeting, GreetingRepository, ServiceError};

#[derive(Debug)]
pub enum RepoError {
    InMemoryError,
}

impl From<RepoError> for ServiceError {
    fn from(_error: RepoError) -> Self {
        ServiceError::UnrecognizedGreetingError
    }
}


pub struct GreetingRepositoryInMemory {
    repo: RwLock<HashMap<usize, GreetingEntity>>
}

impl GreetingRepositoryInMemory {
    pub fn new(map_store: HashMap<usize, GreetingEntity>) -> Self {
        GreetingRepositoryInMemory {
            repo: RwLock::new(map_store)
        }
    }
}

impl GreetingRepository for GreetingRepositoryInMemory {
    fn all(&self) -> Result<Vec<Greeting>, ServiceError> {
        if let Ok(result) = self.repo.read(){
            let guarded_repo = result;
            return Ok(guarded_repo.values().map(|g|Greeting::from(g.clone())).collect::<Vec<_>>());
        }
        Err(ServiceError::from(RepoError::InMemoryError))
    }

    fn store(&mut self, greeting: Greeting) -> Result<(), ServiceError> {
        if let Ok(mut result) = self.repo.write(){

            let key = &result.len() + 1;
            result.insert(key, GreetingEntity::from(greeting));
            return Ok(());
        }
        Err(ServiceError::from(RepoError::InMemoryError))
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct GreetingEntity {
    pub(crate) id: Uuid,
    pub(crate) to: String,
    pub(crate)from: String,
    pub(crate)heading: String,
    pub(crate)message: String,
    pub(crate)created: DateTime<Utc>,
}

impl From<Greeting> for GreetingEntity {
    fn from(greeting: Greeting) -> Self {
        GreetingEntity {
            id: Uuid::default(),
            to: greeting.to,
            from: greeting.from,
            heading: greeting.heading,
            message: greeting.message,
            created: greeting.created,
        }
    }
}

impl From<GreetingEntity> for Greeting {
    fn from(greeting: GreetingEntity) -> Self {
        Greeting {
            to: greeting.to,
            from: greeting.from,
            heading: greeting.heading,
            message: greeting.message,
            created: greeting.created,
        }
    }
}




impl GreetingEntity {
    pub fn new(to: String, from: String, heading: String, message: String) -> Self {
        GreetingEntity {
            id: Uuid::default(),
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
    fn test_greeting_repository() {
        let mut repo = GreetingRepositoryInMemory::new(HashMap::new());

        // Test storing a greeting
        let greeting_entity = GreetingEntity::new(
            String::from("John"),
            String::from("Mary"),
            String::from("Happy Birthday!"),
            String::from("Wishing you a wonderful birthday!")
        );

        repo.store(Greeting::from(greeting_entity.clone())).unwrap();

        // Test retrieving all greetings
        let greetings = repo.all().unwrap().iter().map(|g| GreetingEntity::from(g.clone())).collect::<Vec<_>>();
        assert_eq!(greetings.len(), 1);
        assert_eq!(greetings[0], greeting_entity);

    }
}
