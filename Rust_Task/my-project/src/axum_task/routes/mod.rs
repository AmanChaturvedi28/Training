use axum::Router;

use super::{employee::get_employee, executives::get_executives, health_check, students::get_student};

pub fn get_routes() -> Router {
    let app = Router::new()
        .merge(health_check())
        .merge(get_employee())
        .merge(get_executives())
        .merge(get_student());
    app
}