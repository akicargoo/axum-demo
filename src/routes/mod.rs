mod hello_world;
use hello_world::hello_world;

use axum::{Router, routing::get, body::Body};

pub fn create_routes() -> Router<Body> {
    Router::new()
        .route("/", get(hello_world))

}