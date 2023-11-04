use askama::Template;
use system::{extract::State, response::IntoResponse, Error, Response};

use crate::controllers::HomeController;
use crate::model::post::{Post, PostTemplateData};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    posts: Vec<PostTemplateData>,
}

impl HomeController {
    pub async fn index(State(state): system::State) -> Response<impl IntoResponse> {
        let db = &state.db;

        let posts = sqlx::query_as!(Post, "SELECT * FROM posts LIMIT 10")
            .fetch_all(db.get_pool())
            .await
            .map_err(|e| Error::Database(e))?;

        Ok(state.render(IndexTemplate {
            posts: posts.iter().map(|f| f.into()).collect(),
        }))
    }
}
