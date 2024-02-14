pub mod students_model;

use axum::{routing::post, Router};
use students_model::read_student;
use students_model::read_student_id;
use students_model::delete_student;
use students_model::create_student;
use students_model::update_student;

///this function inclues all the routes that work on student data
pub fn get_student() -> Router {
    let routes = Router::new()
    .route("/read_student", post(read_student))
    .route("/read_student_id/:id", post(read_student_id))
    .route("/delete_student/:id", post(delete_student))
    .route("/update_student", post(update_student))
    .route("/create_student", post(create_student));
    routes
}