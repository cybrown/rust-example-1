use diesel::pg::PgConnection;
use diesel::Connection;

pub fn connect() -> PgConnection {
    PgConnection::establish("postgres://postgres@localhost/postgres").expect("ok")
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
