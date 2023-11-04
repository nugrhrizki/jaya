use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i64,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct PostTemplateData {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&Post> for PostTemplateData {
    fn from(post: &Post) -> Self {
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

impl From<Post> for PostTemplateData {
    fn from(post: Post) -> Self {
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
