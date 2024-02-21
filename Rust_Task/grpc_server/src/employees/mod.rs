use tonic::{Request, Response, Status};

pub mod model;
pub mod service;

use employee::{
    employee_service_server::EmployeeService as employee_service,EmployeeData,EmployeeId,EmployeeResponse
};

use self::{employee::Empty, service::{create_employee, delete_employee, read_all_employee, read_employee, update_employee}};

pub mod employee {
    tonic::include_proto!("emp");
}

#[derive(Debug, Default)]
pub struct EmployeesService {}

#[tonic::async_trait]
impl employee_service for EmployeesService {
    async fn get_employee(&self, request: Request<EmployeeId>) -> Result<Response<EmployeeResponse>, Status> {
        let student_id = request.into_inner().id;
        if let Some(student_data) = read_employee(student_id).await {
            let response = EmployeeResponse {
                key: 2000,
                status: String::from("Fetch successfull"),
                data: student_data,
            };
            Ok(Response::new(response))
        } else {
            let response = EmployeeResponse {
                key: 4004,
                status: String::from("employee not found!!"),
                data: "".to_string(),
            };
            Ok(Response::new(response))
        }
    }

    async fn delete_employee(&self, request: Request<EmployeeId>) -> Result<Response<EmployeeResponse>, Status> {
        let student_id = request.into_inner().id;
        if let Some(deleted_student_data) = delete_employee(student_id).await {
            let response = EmployeeResponse {
                key: 2000,
                status: String::from("delete success"),
                data: deleted_student_data,
            };
            Ok(Response::new(response))
        } else {
            let response = EmployeeResponse {
                key: 4004,
                status: String::from("employee not found"),
                data: "".to_string(),
            };
            Ok(Response::new(response))
        }
    }

    async fn add_employee(&self, request: Request<EmployeeData>) -> Result<Response<EmployeeResponse>, Status> {
        let stud_data = request.into_inner();
        match create_employee(stud_data).await {
            Ok(serialized_student) => {
                let response = EmployeeResponse {
                    key: 2000,
                    status: String::from("Insert success"),
                    data: serialized_student,
                };
                Ok(Response::new(response))
            }
            Err(error_message) => {
                let response = EmployeeResponse {
                    key: 4004,
                    status: error_message,
                    data: "".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }

    async fn update_employee(&self, request: Request<EmployeeData>) -> Result<Response<EmployeeResponse>, Status> {
        let updated_data = request.into_inner();
    
        match update_employee(updated_data).await {
            Ok(updated_student_json) => {
                let response = EmployeeResponse {
                    key: 2000,
                    status: String::from("Update or insert success"),
                    data: updated_student_json,
                };
                Ok(Response::new(response))
            }
            Err(error_message) => {
                let response = EmployeeResponse {
                    key: 4004,
                    status: error_message,
                    data: "".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }

    async fn get_all_employee(&self, request: Request<Empty>) -> Result<Response<EmployeeResponse>, Status> {
        match read_all_employee().await {
            Some(executives) => {
                let response = EmployeeResponse {
                    key: 2000,
                    status: String::from("Fetch successfull"),
                    data: executives,
                };
                Ok(Response::new(response))
            }
            None => {
                let response = EmployeeResponse {
                    key: 2000,
                    status: String::from("No data found"),
                    data: "".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }
}