use askama::Template;
use system::{extract::State, response::IntoResponse, AppState, Response, TemplateUtils};

use crate::data::post::{Post, PostDB};

#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexTemplate {
    posts: Vec<Post>,
}

impl TemplateUtils for Post {}

pub async fn index(State(state): State<AppState>) -> Response<impl IntoResponse> {
    let db = &state.db;

    let posts = PostDB::all(db.get_pool()).await?;

    Ok(state.render(IndexTemplate {
        posts: posts.iter().map(|f| f.into()).collect(),
    }))
}
