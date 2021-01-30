use async_trait::async_trait;
use domain::{DomainError, DomainResult, Post, PostDb};
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, query, PgPool, Postgres};

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
        let res = query!(
            r#"
                SELECT "id", "title", "body", "published"
                FROM "posts"
                WHERE "id" = $1
            "#,
            post_id
        )
        .map(|rec| Post {
            id: rec.id,
            title: rec.title,
            body: rec.body,
            published: rec.published,
        })
        .fetch_optional(&mut tx)
        .await
        .map_err(|err| DomainError::new(format!("failed to select: {}", err).to_owned()))?;
        Ok(res)
    }

    async fn get_posts(&self, show_all: bool) -> DomainResult<Vec<Post>> {
        let mut tx = self.acquire().await?;
        let query = if show_all {
            query!(
                r#"
                SELECT "id", "title", "body", "published"
                FROM "posts"
            "#,
            )
            .map(|rec| Post {
                id: rec.id,
                title: rec.title,
                body: rec.body,
                published: rec.published,
            })
            .fetch_all(&mut tx)
            .await
        } else {
            query!(
                r#"
                    SELECT "id", "title", "body", "published"
                    FROM "posts"
                    WHERE "published" = false
                    "#,
            )
            .map(|rec| Post {
                id: rec.id,
                title: rec.title,
                body: rec.body,
                published: rec.published,
            })
            .fetch_all(&mut tx)
            .await
        };
        let posts = query.map_err(|_| DomainError::new("failed to select".to_owned()))?;
        Ok(posts)
    }

    async fn create_post(&self, title: String, body: String) -> DomainResult<Post> {
        let mut tx = self.acquire().await?;
        let post = query!(
            r#"
                INSERT INTO "posts" ("title", "body")
                VALUES ($1, $2)
                RETURNING "id", "title", "body", "published"
            "#,
            title,
            body
        )
        .map(|rec| Post {
            id: rec.id,
            title: rec.title,
            body: rec.body,
            published: rec.published,
        })
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
        let post = query!(
            r#"
                UPDATE "posts"
                SET "published" = $2
                WHERE "id" = $1
                RETURNING "id", "title", "body", "published"
            "#,
            post_id,
            published,
        )
        .map(|rec| Post {
            id: rec.id,
            title: rec.title,
            body: rec.body,
            published: rec.published,
        })
        .fetch_one(&mut tx)
        .await
        .map_err(|err| DomainError::new(format!("failed to insert post: {}", err).to_owned()))?;
        Ok(Some(post))
    }
}
