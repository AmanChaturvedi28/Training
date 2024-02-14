use axum::{routing::get, Router};

pub fn health_check() -> Router {
    Router::new().route("/health/check", get(root))
}

async fn root() -> String {
    "Everything is good!!".to_string()
}
