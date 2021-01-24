use diesel::pg::PgConnection;
use diesel::Connection;

pub fn connect() -> PgConnection {
    PgConnection::establish("postgres://postgres@localhost/postgres").expect("ok")
}
