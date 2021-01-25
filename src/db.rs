use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

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

#[derive(Debug)]
pub struct DbError;

impl From<diesel::result::Error> for DbError {
    fn from(_: diesel::result::Error) -> Self {
        DbError {}
    }
}
