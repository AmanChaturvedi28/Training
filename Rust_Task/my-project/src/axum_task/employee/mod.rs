use axum::{routing::post, Router};
mod employee_model;
use employee_model::{read_employee,read_employee_id,delete_employee};
use employee_model::update_employee;
use employee_model::create_employee;

///this function inclues all the routes that work on employee data
pub fn get_employee() -> Router{
    let routes = Router::new()
    .route("/read_employee_id/:id", post(read_employee_id))
    .route("/read_employee", post(read_employee))
    .route("/delete_employee/:id", post(delete_employee))
    .route("/update_employee", post(update_employee))
    .route("/create_employee", post(create_employee));

    routes
}