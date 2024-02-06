use async_trait::async_trait;
use chrono::NaiveDateTime;
use derive_more::{Display, Error};
use futures::executor::block_on;
use sqlx::{Error, migrate, Pool, Postgres};
use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use crate::greeting::service::{Greeting, GreetingRepository, ServiceError};

#[async_trait]
impl GreetingRepository for SqliteStudentRepository<Postgres> {
    async fn all(&self) -> Result<Vec<Greeting>, ServiceError> {
        let greetings = sqlx::query_as!
        (GreetingEntity, "SELECT id, \"from\", \"to\", heading, message, created FROM greeting ")
            .fetch_all(&self.pool).await?;

        Ok(greetings.iter().map(|v| Greeting::from(v.clone())).collect::<Vec<_>>())
    }

    async  fn store(&mut self, greeting: Greeting) -> Result<(), ServiceError> {
        let new_greeting = GreetingEntity::from(greeting);
        sqlx::query_as!(GreetingEntity,"INSERT INTO greeting(id, \"from\", \"to\", heading, message, created) VALUES ($1, $2, $3, $4, $5, $6)",
            new_greeting.id, new_greeting.from,new_greeting.to, new_greeting.heading, new_greeting.message, new_greeting.created)
            .fetch_one(&self.pool).await?;

        Ok(())
    }
}


pub struct SqliteStudentRepository<T: sqlx::Database> {
    pool: Pool<T>,
}

impl  SqliteStudentRepository<Postgres> {
    pub async fn new(database_url: &str) -> Result<Self, RepoError> {

        let pool = PgPoolOptions::new()
            .max_connections(100)
            .connect(database_url).await?;
        migrate!("./migrations")
            .run(&pool).await?;
        Ok(SqliteStudentRepository{pool})
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GreetingEntity {
    pub(crate) id: Uuid,
    pub(crate) to: String,
    pub(crate) from: String,
    pub(crate) heading: String,
    pub(crate) message: String,
    pub(crate) created: NaiveDateTime,
}



impl From<Greeting> for GreetingEntity {
    fn from(greeting: Greeting) -> Self {
        GreetingEntity {
            id:  Uuid::now_v7(),
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
            id: Default::default(),
            to,
            from,
            heading,
            message,
            created: NaiveDateTime::default(),
        }
    }
}

#[derive(Debug, Display, Error)]
pub enum RepoError {
    DbError(Error),
    DbMigrationError(MigrateError)
}

impl From<RepoError> for ServiceError {
    fn from(_error: RepoError) -> Self {
        ServiceError::RepoError(_error.to_string())
    }
}
impl From<Error> for ServiceError {
    fn from(_error: Error) -> Self {
        ServiceError::RepoError(_error.to_string())
    }
}
impl From<Error> for RepoError{
    fn from(value: Error) -> Self {
        RepoError::DbError(value)
    }
}

impl From<MigrateError> for RepoError {
    fn from(value: MigrateError) -> Self {
        RepoError::DbMigrationError(value)
    }
}
