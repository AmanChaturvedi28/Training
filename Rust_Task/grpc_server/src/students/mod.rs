use tonic::{Request, Response, Status};

pub mod model;
pub mod service;
use self::{service::{create_student, delete_student, read_all_student, read_student, update_student}, student::Empty};

use student::{
        student_service_server::StudentService as student_service, StudentData, StudentId, StudentResponse,
    };

pub mod student {
    tonic::include_proto!("stud");
}

#[derive(Debug, Default)]
pub struct StudentsService {}

#[tonic::async_trait]
impl student_service for StudentsService {
    async fn get_student(&self, request: Request<StudentId>) -> Result<Response<StudentResponse>, Status> {
        let student_id = request.into_inner().id;
        if let Some(student_data) = read_student(student_id).await {
            let response = StudentResponse {
                key: 2000,
                status: String::from("Fetch successfull"),
                data: student_data,
            };
            Ok(Response::new(response))
        } else {
            let response = StudentResponse {
                key: 4004,
                status: String::from("Student not found!!"),
                data: "".to_string(),
            };
            Ok(Response::new(response))
        }
    }

    async fn add_student(&self, request: Request<StudentData>) -> Result<Response<StudentResponse>, Status> {
        let stud_data = request.into_inner();
        match create_student(stud_data).await {
            Ok(serialized_student) => {
                let response = StudentResponse {
                    key: 2000,
                    status: String::from("Insert success"),
                    data: serialized_student,
                };
                Ok(Response::new(response))
            }
            Err(error_message) => {
                let response = StudentResponse {
                    key: 4004,
                    status: error_message,
                    data: "".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }

    async fn delete_student(&self, request: Request<StudentId>) -> Result<Response<StudentResponse>, Status> {
        let student_id = request.into_inner().id;
        if let Some(deleted_student_data) = delete_student(student_id).await {
            let response = StudentResponse {
                key: 2000,
                status: String::from("delete success"),
                data: deleted_student_data,
            };
            Ok(Response::new(response))
        } else {
            let response = StudentResponse {
                key: 4004,
                status: String::from("Student not found"),
                data: "".to_string(),
            };
            Ok(Response::new(response))
        }
    }

    async fn update_student(&self, request: Request<StudentData>) -> Result<Response<StudentResponse>, Status> {
        let updated_data = request.into_inner();
    
        match update_student(updated_data).await {
            Ok(updated_student_json) => {
                let response = StudentResponse {
                    key: 2000,
                    status: String::from("Update or insert success"),
                    data: updated_student_json,
                };
                Ok(Response::new(response))
            }
            Err(error_message) => {
                let response = StudentResponse {
                    key: 4004,
                    status: error_message,
                    data: "".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }

    async fn get_all_student(&self, request: Request<Empty>) -> Result<Response<StudentResponse>, Status> {
        match read_all_student().await {
            Some(executives) => {
                let response = StudentResponse {
                    key: 2000,
                    status: String::from("Fetch successfull"),
                    data: executives,
                };
                Ok(Response::new(response))
            }
            None => {
                let response = StudentResponse {
                    key: 2000,
                    status: String::from("No data found"),
                    data: "".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }
}
