use askama::Template;
use serde::{Deserialize, Serialize};
use serde_json::json;
use system::{
    extract::{Path, State},
    response::IntoResponse,
    AppState, Json, Response,
};

use crate::data::post::{Post, PostDB};

#[derive(Deserialize, Serialize)]
pub struct CreatePayload {
    pub title: String,
    pub body: String,
}

#[derive(Template)]
#[template(path = "pages/post/index.html")]
struct PostTemplate {
    posts: Vec<Post>,
}

pub async fn index(State(state): State<AppState>) -> Response<impl IntoResponse> {
    let db = &state.db;

    let posts = PostDB::all(db.get_pool()).await?;

    Ok(state.render(PostTemplate {
        posts: posts.iter().map(|f| f.into()).collect(),
    }))
}

#[derive(Template)]
#[template(path = "pages/post/show.html")]
struct ShowPostTemplate {
    post: Post,
}
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Response<impl IntoResponse> {
    let db = &state.db;

    let post = PostDB::find(db.get_pool(), id).await?;

    Ok(state.render(ShowPostTemplate { post: post.into() }))
}

#[derive(Template)]
#[template(path = "pages/post/create.html")]
struct CreateTemplate;
pub async fn create(State(state): State<AppState>) -> Response<impl IntoResponse> {
    Ok(state.render(CreateTemplate))
}

#[derive(Template)]
#[template(path = "pages/post/edit.html")]
struct EditPostTemplate {
    post: Post,
}
pub async fn edit(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Response<impl IntoResponse> {
    let db = &state.db;

    let post = PostDB::find(db.get_pool(), id).await?;

    Ok(state.render(EditPostTemplate { post: post.into() }))
}

pub async fn save(
    State(state): State<AppState>,
    Json(payload): Json<CreatePayload>,
) -> Response<impl IntoResponse> {
    let db = &state.db;

    let post = PostDB::insert(
        db.get_pool(),
        PostDB {
            title: Some(payload.title),
            body: Some(payload.body),
            ..PostDB::default()
        },
    )
    .await?;

    Ok(Json(json!({
        "post": post,
    })))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<CreatePayload>,
) -> Response<impl IntoResponse> {
    let db = &state.db;

    let post = PostDB::update(
        db.get_pool(),
        PostDB {
            title: Some(payload.title),
            body: Some(payload.body),
            ..PostDB::default()
        },
        id,
    )
    .await?;

    Ok(Json(json!({
        "post": post,
    })))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Response<impl IntoResponse> {
    let db = &state.db;

    let post = PostDB::delete(db.get_pool(), id).await?;

    Ok(Json(json!({
        "post": post,
    })))
}
