use system::{routing::get, Router};

use crate::controllers::{home_controller::HomeController, post_controller::PostController};

pub fn web() -> Router {
    Router::new()
        .route("/", get(HomeController::index))
        .route("/post", get(PostController::index))
}
