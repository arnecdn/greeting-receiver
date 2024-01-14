use std::collections::HashMap;
use std::sync::RwLock;

use chrono::{DateTime, Utc};
#[derive(Debug)]
pub enum RepoError {
    InMemoryError,
}


pub trait GreetingRepository {
    // Get all greetings
    fn all(&self) -> Result<Vec<GreetingEntity>, RepoError>;

    // Create a new student
     fn store(&self, student: GreetingEntity) -> Result<(), RepoError>;
}

pub struct GreetingRepositoryInMemory {
    repo: RwLock<HashMap<usize, GreetingEntity>>
}

impl GreetingRepositoryInMemory {
    pub fn new() -> Self {
        GreetingRepositoryInMemory {
            repo: RwLock::new(HashMap::new())
        }
    }
}

impl GreetingRepository for GreetingRepositoryInMemory {
    fn all(&self) -> Result<Vec<GreetingEntity>, RepoError> {
        if let Ok(result) = self.repo.read(){
            let guarded_repo = result;
            return Ok(guarded_repo.values().map(|f|f.clone()).collect::<Vec<_>>());
        }
        Err(RepoError::InMemoryError)
    }

    fn store(&self, greeting: GreetingEntity) -> Result<(), RepoError> {
        if let Ok(mut result) = self.repo.write(){

            let key = &result.len() + 1;
            result.insert(key, greeting);
            return Ok(());
        }
        Err(RepoError::InMemoryError)
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct GreetingEntity {
    to: String,
    from: String,
    heading: String,
    message: String,
    created: DateTime<Utc>,
}






impl GreetingEntity {
    pub fn new(to: String, from: String, heading: String, message: String) -> Self {
        GreetingEntity {
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
        let repo = GreetingRepositoryInMemory::new();

        // Test storing a greeting
        let greeting = GreetingEntity::new(
            String::from("John"),
            String::from("Mary"),
            String::from("Happy Birthday!"),
            String::from("Wishing you a wonderful birthday!")
        );

        repo.store(greeting.clone()).unwrap();

        // Test retrieving all greetings
        let greetings = repo.all().unwrap();
        assert_eq!(greetings.len(), 1);
        assert_eq!(greetings[0], greeting);

    }
}
