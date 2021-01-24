use crate::db::Post;
use crate::schema::posts;
use diesel::PgConnection;

use diesel::prelude::*;

pub struct DieselPostDb(PgConnection);

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

impl DieselPostDb {
    pub fn new(pg: PgConnection) -> Self {
        Self(pg)
    }

    pub fn get_posts(&self) -> QueryResult<Vec<Post>> {
        posts::dsl::posts
            .filter(posts::dsl::published.eq(true))
            .load::<Post>(&self.0)
    }

    pub fn insert_post(&self, title: String, body: String) -> QueryResult<Post> {
        let insert_post = InsertPost { title, body };
        diesel::insert_into(posts::table)
            .values(&insert_post)
            .get_result::<Post>(&self.0)
    }

    pub fn update_post(&self, values: UpdatePost) -> QueryResult<Post> {
        diesel::update(posts::table.filter(posts::dsl::published.eq(true)))
            .set(values)
            .get_result::<Post>(&self.0)
    }
}
