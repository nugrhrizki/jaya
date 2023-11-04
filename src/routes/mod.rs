use system::{routing::get, Router};

use crate::controllers::{HomeController, PostController};

pub fn web() -> Router {
    Router::new().route("/", get(HomeController::index)).nest(
        "/post",
        Router::new()
            .route("/", get(PostController::index))
            .route(
                "/create",
                get(PostController::create).post(PostController::save),
            )
            .route(
                "/:id/edit",
                get(PostController::edit).put(PostController::update),
            )
            .route(
                "/:id",
                get(PostController::show).delete(PostController::delete),
            ),
    )
}
