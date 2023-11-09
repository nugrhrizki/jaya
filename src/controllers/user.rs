use askama::Template;
use system::{extract::State, response::IntoResponse, AppState, Response};

#[derive(Template)]
#[template(path = "pages/user/index.html")]
struct IndexTemplate;

pub async fn index(State(state): State<AppState>) -> Response<impl IntoResponse> {
    Ok(state.render(IndexTemplate))
}
