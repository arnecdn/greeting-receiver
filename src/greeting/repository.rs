use std::error::Error;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::sqlite::SqlitePool;
use sqlx::sqlx_macros::migrate;
use validator_derive::Validate;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("NotFound")]
    NotFound,

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}


pub trait GreetingRepository {
    // Get all greetings
    fn all(&self) -> Result<Vec<Greeting>, sqlx::Error>;

    // Get a specific student by id
     fn get(&self, id: &str) -> Result<Greeting, sqlx::Error>;

    // Create a new student
     fn create(&self, student: &Greeting) -> Result<Greeting, sqlx::Error>;

    // Delete a student by id
     fn delete(&self, id: &str) -> Result<Greeting, sqlx::Error>;
}

#[derive(Validate,Debug, FromRow, PartialEq, Eq)]
pub struct Greeting {
    #[validate(length(min = 1, max = 50))]
    to: String,
    #[validate(length(min = 1, max = 50))]
    from: String,
    #[validate(length(min = 1, max = 50))]
    heading: String,
    #[validate(length(min = 1, max = 200))]
    message: String,
    created: DateTime<Utc>,
}

pub struct SqliteStudentRepository {
    pool: SqlitePool,
}
impl SqliteStudentRepository {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        migrate!("./migrations")
            .run(&pool)
            .await?;

        Ok(SqliteStudentRepository { pool })
    }
}
