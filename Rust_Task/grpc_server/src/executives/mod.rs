use tonic::{Request, Response, Status};

pub mod model;
pub mod service;

use executive::{
    executive_service_server::ExecutiveService as executive_service,ExecutiveData,ExecutiveId,ExecutiveResponse
};

use self::{executive::Empty, service::{create_executive, delete_executive, read_all_executive, read_executive, update_executive}};

pub mod executive {
    tonic::include_proto!("exec");
}


#[derive(Debug, Default)]
pub struct ExecutivesService {}

#[tonic::async_trait]
impl executive_service for ExecutivesService{
    async fn get_executive(&self, request: Request<ExecutiveId>) -> Result<Response<ExecutiveResponse>, Status> {
        let student_id = request.into_inner().id;
        if let Some(student_data) = read_executive(student_id).await {
            let response = ExecutiveResponse {
                key: 2000,
                status: String::from("Fetch successfull"),
                data: student_data,
            };
            Ok(Response::new(response))
        } else {
            let response = ExecutiveResponse {
                key: 4004,
                status: String::from("executive not found!!"),
                data: "".to_string(),
            };
            Ok(Response::new(response))
        }
    }

    async fn delete_executive(&self, request: Request<ExecutiveId>) -> Result<Response<ExecutiveResponse>, Status> {
        let student_id = request.into_inner().id;
        if let Some(deleted_student_data) = delete_executive(student_id).await {
            let response = ExecutiveResponse {
                key: 2000,
                status: String::from("delete success"),
                data: deleted_student_data,
            };
            Ok(Response::new(response))
        } else {
            let response = ExecutiveResponse {
                key: 4004, // Assuming 404 for not found
                status: String::from("executive not found"),
                data: "".to_string(),
            };
            Ok(Response::new(response))
        }
    }

    async fn add_executive(&self, request: Request<ExecutiveData>) -> Result<Response<ExecutiveResponse>, Status> {
        let stud_data = request.into_inner();
        match create_executive(stud_data).await {
            Ok(serialized_student) => {
                let response = ExecutiveResponse {
                    key: 2000,
                    status: String::from("Insert success"),
                    data: serialized_student,
                };
                Ok(Response::new(response))
            }
            Err(error_message) => {
                let response = ExecutiveResponse {
                    key: 4004,
                    status: error_message,
                    data: "".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }

    async fn update_executive(&self, request: Request<ExecutiveData>) -> Result<Response<ExecutiveResponse>, Status> {
        let updated_data = request.into_inner();
    
        match update_executive(updated_data).await {
            Ok(updated_student_json) => {
                let response = ExecutiveResponse {
                    key: 2000,
                    status: String::from("Update or insert success"),
                    data: updated_student_json,
                };
                Ok(Response::new(response))
            }
            Err(error_message) => {
                let response = ExecutiveResponse {
                    key: 4004,
                    status: error_message,
                    data: "".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }

    async fn get_all_executive(&self, request: Request<Empty>) -> Result<Response<ExecutiveResponse>, Status> {
        match read_all_executive().await {
            Some(executives) => {
                let response = ExecutiveResponse {
                    key: 2000,
                    status: String::from("Fetch successfull"),
                    data: executives,
                };
                Ok(Response::new(response))
            }
            None => {
                let response = ExecutiveResponse {
                    key: 2000,
                    status: String::from("No data found"),
                    data: "".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }
}