pub mod executives_model;

use axum::{routing::post, Router};
use executives_model::delete_executive;
use executives_model::read_executive;
use executives_model::read_executive_id;
use executives_model::update_executive;
use executives_model::create_executive;

pub fn get_executives() -> Router {
    let routes = Router::new()
        .route("/read_executive_id/:id", post(read_executive_id))
        .route("/read_executive", post(read_executive))
        .route("/delete_executive/:id", post(delete_executive))
        .route("/update_executive", post(update_executive))
        .route("/create_executive", post(create_executive));

    routes
}
