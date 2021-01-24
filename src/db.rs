use diesel::pg::PgConnection;
use diesel::Connection;
use serde::Serialize;

pub fn connect() -> PgConnection {
    PgConnection::establish("postgres://postgres@localhost/postgres").expect("ok")
}

#[derive(Queryable, Default, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
