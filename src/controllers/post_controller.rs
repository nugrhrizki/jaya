use askama::Template;
use system::{error::Error, extract::State, response::IntoResponse, Response};

use crate::model::post::Post;

pub struct PostController;

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    posts: Vec<Post>,
}

impl PostController {
    pub async fn index(State(state): system::State) -> Response<impl IntoResponse> {
        let db = &state.db;

        let posts: Vec<Post> = db
            .get(Post::default())
            .await
            .map_err(|e| Error::Database(e))?;

        Ok(state.render(PostTemplate { posts }))
    }
}
