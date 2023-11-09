use system::{
    routing::{delete, get},
    Router,
};

use crate::controllers::{home, post, user};

pub fn router() -> Router {
    Router::new()
        .route("/", get(home::index))
        .nest(
            "/post",
            Router::new()
                .route("/", get(post::index))
                .route("/create", get(post::create).post(post::save))
                .route("/:id/edit", get(post::edit).put(post::update))
                .route("/:id/delete", delete(post::delete))
                .route("/:id", get(post::show)),
        )
        .route("/user", get(user::index))
}
