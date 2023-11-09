use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::{FromRow, Pool, Postgres};
use system::{Error, Result};

#[derive(Serialize, Deserialize, FromRow, Default)]
pub struct PostDB {
    pub id: i64,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl PostDB {
    pub async fn all(pool: &Pool<Postgres>) -> Result<Vec<Self>> {
        sqlx::query_as("SELECT * FROM posts LIMIT 10")
            .fetch_all(pool)
            .await
            .map_err(|e| Error::Database(e))
    }

    pub async fn find(pool: &Pool<Postgres>, id: i64) -> Result<Self> {
        sqlx::query_as("SELECT * FROM posts WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e))
    }

    pub async fn insert(pool: &Pool<Postgres>, payload: PostDB) -> Result<Self> {
        sqlx::query_as(
            "INSERT INTO posts (title, body, created_at, updated_at) VALUES ($1, $2, current_timestamp, current_timestamp) RETURNING *",
        )
        .bind(payload.title)
        .bind(payload.body)
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))
    }

    pub async fn update(pool: &Pool<Postgres>, payload: PostDB, id: i64) -> Result<Self> {
        sqlx::query_as(
            "UPDATE posts SET title = $1, body = $2, updated_at = current_timestamp WHERE id = $3 RETURNING *",
        )
        .bind(payload.title)
        .bind(payload.body)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))
    }

    pub async fn delete(pool: &Pool<Postgres>, id: i64) -> Result<Self> {
        sqlx::query_as("DELETE FROM posts WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&PostDB> for Post {
    fn from(post: &PostDB) -> Self {
        Self {
            id: post.id,
            title: post.title.clone().unwrap_or_default(),
            body: post.body.clone().unwrap_or_default(),
            created_at: post
                .created_at
                .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_default(),
            updated_at: post
                .updated_at
                .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_default(),
        }
    }
}

impl From<PostDB> for Post {
    fn from(post: PostDB) -> Self {
        Self {
            id: post.id,
            title: post.title.unwrap_or_default(),
            body: post.body.unwrap_or_default(),
            created_at: post
                .created_at
                .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_default(),
            updated_at: post
                .updated_at
                .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_default(),
        }
    }
}
