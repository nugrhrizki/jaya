mod controllers;
mod data;
mod routes;

use system::System;

fn main() {
    if let Err(e) = System::with_router(routes::setup()).prefork(0).run() {
        panic!("{e}");
    }
}
