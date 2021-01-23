use crate::db::Post;
use crate::schema::posts::dsl::*;
use diesel::PgConnection;

use diesel::prelude::*;

pub struct PostsDao(PgConnection);

impl PostsDao {
    pub fn new(pg: PgConnection) -> Self {
        Self(pg)
    }

    pub fn get_posts(&self) -> QueryResult<Vec<Post>> {
        posts.filter(published.eq(true)).load::<Post>(&self.0)
    }
}
