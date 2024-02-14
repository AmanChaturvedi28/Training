use axum::{routing::get, Router};

///this function checks wether the server is working properly or not
pub fn health_check() -> Router {
    Router::new().route("/health/check", get(root))
}

///handler for health check
async fn root() -> String {
    "Everything is good!!".to_string()
}
