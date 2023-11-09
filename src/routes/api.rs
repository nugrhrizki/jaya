use serde_json::json;
use system::{routing::get, Json, Router};

pub fn router() -> Router {
    Router::new().route(
        "/",
        get(|| async {
            Json(json!({
                "message": "Hello from the API!"
            }))
        }),
    )
}
