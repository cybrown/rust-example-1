use crate::db::DbError;
use crate::db::PgConnectionFactory;
use crate::schema::posts;

use diesel::prelude::*;

#[derive(Clone)]
pub struct DieselPostDb {
    pg_connection_factory: PgConnectionFactory,
}

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

pub struct GetPostsCriteria {
    pub published: Option<bool>,
}

impl DieselPostDb {
    pub fn new(pg_connection_factory: PgConnectionFactory) -> Self {
        Self {
            pg_connection_factory,
        }
    }

    pub fn get_posts(&self, criteria: GetPostsCriteria) -> Result<Vec<Post>, DbError> {
        let mut query = posts::dsl::posts.into_boxed();
        if let Some(published) = criteria.published {
            query = query.filter(posts::dsl::published.eq(published))
        };
        Ok(query.load::<Post>(&*self.pg_connection_factory.get_connection()?)?)
    }

    pub fn insert_post(&self, title: String, body: String) -> Result<Post, DbError> {
        let insert_post = InsertPost { title, body };
        Ok(diesel::insert_into(posts::table)
            .values(&insert_post)
            .get_result::<Post>(&*self.pg_connection_factory.get_connection()?)?)
    }

    pub fn update_post(&self, values: UpdatePost) -> Result<Post, DbError> {
        Ok(
            diesel::update(posts::table.filter(posts::dsl::published.eq(true)))
                .set(values)
                .get_result::<Post>(&*self.pg_connection_factory.get_connection()?)?,
        )
    }
}
