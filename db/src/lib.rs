use async_trait::async_trait;
use domain::{DomainError, DomainResult, Post, PostDb, PostUpdates};
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    query, PgPool, Row,
};

#[derive(Clone)]
pub struct SqlxPostDb {
    pool: PgPool,
}

pub async fn connect() -> Result<SqlxPostDb, DomainError> {
    let pool = PgPoolOptions::new()
        .max_connections(16)
        .connect("postgres://postgres@localhost/postgres")
        .await
        .map_err(|_| DomainError::new("failed to select".to_owned()))?;

    // Make a simple query to return the given parameter
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await
        .map_err(|_| DomainError::new("failed to select".to_owned()))?;

    assert_eq!(row.0, 150);

    Ok(SqlxPostDb { pool })
}

#[async_trait]
impl PostDb for SqlxPostDb {
    async fn get_post_by_id(&self, post_id: i32) -> DomainResult<Option<Post>> {
        let mut tx = self
            .pool
            .acquire()
            .await
            .map_err(|_| DomainError::new("failed to select".to_owned()))?;
        let res = query("SELECT id, title, body, published FROM posts where id = ?")
            .bind(post_id)
            .fetch_optional(&mut tx)
            .await
            .map_err(|_| DomainError::new("failed to select".to_owned()))?;
        Ok(res.map(|r| Post {
            id: r.try_get(0).unwrap(),
            title: r.try_get(1).unwrap(),
            body: r.try_get(2).unwrap(),
            published: r.try_get(3).unwrap(),
        }))
    }

    async fn get_posts(&self, show_all: bool) -> DomainResult<Vec<Post>> {
        let mut tx = self
            .pool
            .acquire()
            .await
            .map_err(|_| DomainError::new("failed to select".to_owned()))?;
        let posts = query("SELECT id, title, body, published FROM posts WHERE published = false")
            .map(|row: PgRow| Post {
                id: row.try_get(0).unwrap(),
                title: row.try_get(1).unwrap(),
                body: row.try_get(2).unwrap(),
                published: row.try_get(3).unwrap(),
            })
            .fetch_all(&mut tx)
            .await
            .map_err(|_| DomainError::new("failed to select".to_owned()))?;
        Ok(posts)
    }

    async fn create_post(&self, title: String, body: String) -> DomainResult<Post> {
        todo!()
    }

    async fn update_post(&self, post_id: i32, updates: PostUpdates) -> DomainResult<Option<Post>> {
        todo!()
    }
}
