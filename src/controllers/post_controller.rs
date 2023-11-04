use askama::Template;
use serde::{Deserialize, Serialize};
use serde_json::json;
use system::{
    extract::{Path, State},
    response::IntoResponse,
    Error, Json, Response,
};

use crate::model::post::{Post, PostTemplateData};

pub struct PostController;

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    posts: Vec<PostTemplateData>,
}

#[derive(Template)]
#[template(path = "post/show.html")]
struct ShowPostTemplate {
    post: PostTemplateData,
}

#[derive(Template)]
#[template(path = "post/edit.html")]
struct EditPostTemplate {
    post: PostTemplateData,
}

#[derive(Template)]
#[template(path = "post/create.html")]
struct CreateTemplate;

#[derive(Deserialize, Serialize)]
pub struct CreatePayload {
    pub title: String,
    pub body: String,
}

impl PostController {
    pub async fn index(State(state): system::State) -> Response<impl IntoResponse> {
        let db = &state.db;

        let posts = sqlx::query_as!(Post, "SELECT * FROM posts LIMIT 10")
            .fetch_all(db.get_pool())
            .await
            .map_err(|e| Error::Database(e))?;

        Ok(state.render(PostTemplate {
            posts: posts.iter().map(|f| f.into()).collect(),
        }))
    }

    pub async fn show(
        State(state): system::State,
        Path(id): Path<i64>,
    ) -> Response<impl IntoResponse> {
        let db = &state.db;

        let post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", id)
            .fetch_one(db.get_pool())
            .await
            .map_err(|e| Error::Database(e))?;

        Ok(state.render(ShowPostTemplate { post: post.into() }))
    }

    pub async fn create(State(state): system::State) -> Response<impl IntoResponse> {
        Ok(state.render(CreateTemplate))
    }

    pub async fn edit(
        State(state): system::State,
        Path(id): Path<i64>,
    ) -> Response<impl IntoResponse> {
        let db = &state.db;

        let post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", id)
            .fetch_one(db.get_pool())
            .await
            .map_err(|e| Error::Database(e))?;

        Ok(state.render(EditPostTemplate { post: post.into() }))
    }

    pub async fn save(
        State(state): system::State,
        Json(payload): Json<CreatePayload>,
    ) -> Response<impl IntoResponse> {
        let db = &state.db;

        let post = sqlx::query_as!(
            Post,
            "INSERT INTO posts (title, body, created_at, updated_at) VALUES ($1, $2, current_timestamp, current_timestamp) RETURNING *",
            payload.title,
            payload.body
        )
        .fetch_one(db.get_pool())
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(Json(json!({
            "post": post,
        })))
    }

    pub async fn update(
        State(state): system::State,
        Path(id): Path<i64>,
        Json(payload): Json<CreatePayload>,
    ) -> Response<impl IntoResponse> {
        let db = &state.db;

        let post = sqlx::query_as!(
            Post,
            "UPDATE posts SET title = $1, body = $2 WHERE id = $3 RETURNING *",
            payload.title,
            payload.body,
            id
        )
        .fetch_one(db.get_pool())
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(Json(json!({
            "post": post,
        })))
    }

    pub async fn delete(
        State(state): system::State,
        Path(id): Path<i64>,
    ) -> Response<impl IntoResponse> {
        let db = &state.db;

        let post = sqlx::query_as!(Post, "DELETE FROM posts WHERE id = $1 RETURNING *", id)
            .fetch_one(db.get_pool())
            .await
            .map_err(|e| Error::Database(e))?;

        Ok(Json(json!({
            "post": post,
        })))
    }
}
