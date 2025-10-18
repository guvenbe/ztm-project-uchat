use diesel::{Connection, ConnectionError, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{error::Error, time::Duration};

use crate::error::QueryError;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

#[derive(Clone, Debug)]
pub struct PgConnectionManager {
    url: String,
}

impl PgConnectionManager {
    fn new<S: Into<String>>(url: S) -> Self {
        Self { url: url.into() }
    }
}

#[async_trait::async_trait]
impl bb8::ManageConnection for PgConnectionManager {
    type Connection = PgConnection;
    type Error = ConnectionError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let url = self.url.clone();
        tokio::task::spawn_blocking(move || PgConnection::establish(&url))
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?
    }

    async fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        // Optionally run a simple query here. Keep it lightweight for now.
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}

type AsyncConnectionManager = PgConnectionManager;

pub type AsyncConnection<'a> = bb8::PooledConnection<'a, AsyncConnectionManager>;
pub type OwnedAsyncConnection = bb8::PooledConnection<'static, AsyncConnectionManager>;

#[derive(Clone, Debug)]
pub struct AsyncConnectionPool(bb8::Pool<AsyncConnectionManager>);

impl AsyncConnectionPool {
    pub async fn new<S: AsRef<str>>(url: S) -> Result<Self, QueryError> {
        let pool = new_async_pool(url).await?;
        {
            // check connection
            let _ = pool
                .0
                .get()
                .await
                .map_err(|e| QueryError::Connection(e.to_string()))?;
        }
        Ok(pool)
    }

    pub async fn get(&self) -> Result<AsyncConnection, QueryError> {
        self.0
            .get()
            .await
            .map_err(|e| QueryError::Connection(e.to_string()))
    }

    pub async fn get_owned(&self) -> Result<OwnedAsyncConnection, QueryError> {
        self.0
            .get_owned()
            .await
            .map_err(|e| QueryError::Connection(e.to_string()))
    }

    pub fn state(&self) -> bb8::State {
        self.0.state()
    }
}

/// Run database migrations
pub fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::pg::Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

/// Connect to the database
pub fn connect<S: AsRef<str>>(url: S) -> Result<PgConnection, ConnectionError> {
    use diesel::Connection;
    let url = url.as_ref();
    PgConnection::establish(url)
}

/// Usage:
/// ```
/// let async_pool = new_async_pool("postgres://login@localhost/sample").await;
/// let conn = &mut async_pool.get().await?;
/// ```
pub async fn new_async_pool<S: AsRef<str>>(url: S) -> Result<AsyncConnectionPool, QueryError> {
    let url = url.as_ref();
    let manager = PgConnectionManager::new(url.to_string());
    bb8::Pool::builder()
        .test_on_check_out(true)
        .connection_timeout(Duration::from_secs(10))
        .build(manager)
        .await
        .map(AsyncConnectionPool)
        .map_err(|e| QueryError::Pool(e.to_string()))
}
