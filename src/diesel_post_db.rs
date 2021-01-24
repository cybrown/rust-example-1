use crate::db::DbError;
use crate::db::PgConnectionFactory;
use crate::schema::posts;

use diesel::prelude::*;

pub struct DieselPostDb(PgConnectionFactory);

#[derive(Insertable)]
#[table_name = "posts"]
struct InsertPost {
    pub title: String,
    pub body: String,
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct UpdatePost<'a> {
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub published: Option<bool>,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl DieselPostDb {
    pub fn new(pg: PgConnectionFactory) -> Self {
        Self(pg)
    }

    pub fn get_posts(&self) -> Result<Vec<Post>, DbError> {
        Ok(posts::dsl::posts
            .filter(posts::dsl::published.eq(true))
            .load::<Post>(&*self.0.get_connection()?)?)
    }

    pub fn insert_post(&self, title: String, body: String) -> Result<Post, DbError> {
        let insert_post = InsertPost { title, body };
        Ok(diesel::insert_into(posts::table)
            .values(&insert_post)
            .get_result::<Post>(&*self.0.get_connection()?)?)
    }

    pub fn update_post(&self, values: UpdatePost) -> Result<Post, DbError> {
        Ok(
            diesel::update(posts::table.filter(posts::dsl::published.eq(true)))
                .set(values)
                .get_result::<Post>(&*self.0.get_connection()?)?,
        )
    }
}
