use async_trait::async_trait;
use domain::{DomainError, DomainResult, Post, PostDb};
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, query_as, PgPool, Postgres};
use std::time::Duration;

#[derive(Clone)]
pub struct SqlxPostDb {
    pool: PgPool,
}

pub async fn connect(
    uri: &str,
    min_conn: u32,
    max_conn: u32,
    max_lifetime: Duration,
) -> Result<SqlxPostDb, DomainError> {
    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .min_connections(min_conn)
        .max_lifetime(max_lifetime)
        .connect(uri)
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

impl SqlxPostDb {
    async fn acquire(&self) -> DomainResult<PoolConnection<Postgres>> {
        self.pool
            .acquire()
            .await
            .map_err(|_| DomainError::new("failed to select".to_owned()))
    }
}

#[async_trait]
impl PostDb for SqlxPostDb {
    async fn get_post_by_id(&self, post_id: i32) -> DomainResult<Option<Post>> {
        let mut tx = self.acquire().await?;
        let res = query_as!(
            Post,
            r#"
                SELECT "id", "title", "body", "published"
                FROM "posts"
                WHERE "id" = $1
            "#,
            post_id
        )
        .fetch_optional(&mut tx)
        .await
        .map_err(|err| DomainError::new(format!("failed to select: {}", err).to_owned()))?;
        Ok(res)
    }

    async fn get_posts(&self, show_all: bool) -> DomainResult<Vec<Post>> {
        let mut tx = self.acquire().await?;
        let query = if show_all {
            query_as!(
                Post,
                r#"
                SELECT "id", "title", "body", "published"
                FROM "posts"
            "#,
            )
            .fetch_all(&mut tx)
            .await
        } else {
            query_as!(
                Post,
                r#"
                    SELECT "id", "title", "body", "published"
                    FROM "posts"
                    WHERE "published" = true
                    "#,
            )
            .fetch_all(&mut tx)
            .await
        };
        let posts = query.map_err(|_| DomainError::new("failed to select".to_owned()))?;
        Ok(posts)
    }

    async fn create_post(&self, title: String, body: String) -> DomainResult<Post> {
        let mut tx = self.acquire().await?;
        let post = query_as!(
            Post,
            r#"
                INSERT INTO "posts" ("title", "body")
                VALUES ($1, $2)
                RETURNING "id", "title", "body", "published"
            "#,
            title,
            body
        )
        .fetch_one(&mut tx)
        .await
        .map_err(|err| DomainError::new(format!("failed to insert post: {}", err).to_owned()))?;
        Ok(post)
    }

    async fn post_set_published(
        &self,
        post_id: i32,
        published: bool,
    ) -> DomainResult<Option<Post>> {
        let mut tx = self.acquire().await?;
        let post = query_as!(
            Post,
            r#"
                UPDATE "posts"
                SET "published" = $2
                WHERE "id" = $1
                RETURNING "id", "title", "body", "published"
            "#,
            post_id,
            published,
        )
        .fetch_one(&mut tx)
        .await
        .map_err(|err| DomainError::new(format!("failed to insert post: {}", err).to_owned()))?;
        Ok(Some(post))
    }
}
