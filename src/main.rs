mod controllers;
mod model;
mod routes;

use system::System;

#[tokio::main]
async fn main() {
    let app = System::with_router(routes::web());

    if let Err(e) = app.run().await {
        panic!("{e}");
    }
}
