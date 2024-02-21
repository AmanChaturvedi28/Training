use employees::{employee::employee_service_server::EmployeeServiceServer, EmployeesService};
use students::{student::student_service_server::StudentServiceServer, StudentsService};
use executives::{executive::executive_service_server::ExecutiveServiceServer, ExecutivesService};
use tonic::transport::Server;

mod employees;
mod executives;
mod students;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let student_server = StudentsService::default();
    let employee_server = EmployeesService::default();
    let executive_server = ExecutivesService::default();
    println!("Server Running");
    Server::builder()
        .add_service(StudentServiceServer::new(student_server))
        .add_service(EmployeeServiceServer::new(employee_server))
        .add_service(ExecutiveServiceServer::new(executive_server))
        .serve(addr)
        .await
        .unwrap();
}
