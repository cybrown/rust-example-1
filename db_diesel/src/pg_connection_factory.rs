use std::time::Duration;

use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

#[derive(Clone)]
pub struct PgConnectionFactory {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PgConnectionFactory {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub fn get_connection(
        &self,
    ) -> Result<
        diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
        DbError,
    > {
        self.pool.get().map_err(|_| DbError {})
    }
}

pub fn create_pg_pool() -> Pool<ConnectionManager<PgConnection>> {
    Pool::builder()
        .min_idle(Some(0))
        .max_size(16)
        .idle_timeout(Some(Duration::from_secs(60)))
        .build(ConnectionManager::<PgConnection>::new(
            "postgres://postgres@localhost/postgres",
        ))
        .expect("failed to create connexion pool")
}

#[derive(Debug)]
pub struct DbError;

impl From<diesel::result::Error> for DbError {
    fn from(_: diesel::result::Error) -> Self {
        DbError {}
    }
}
