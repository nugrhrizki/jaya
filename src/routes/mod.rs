mod api;
mod web;

use system::{panic_handler, Router};
use tower_http::catch_panic::CatchPanicLayer;

pub fn setup() -> Router {
    Router::new()
        .nest("/api", api::router())
        .nest("/", web::router())
        .layer(CatchPanicLayer::custom(panic_handler))
}
