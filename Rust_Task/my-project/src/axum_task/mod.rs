pub mod health_check;
pub mod employee;
pub mod routes;
pub mod executives;
pub mod students;
pub mod middlewares;

use std::{fs, sync::{Arc, RwLock}};
use lazy_static::lazy_static;
use health_check::health_check;
use routes::get_routes;
use middlewares::get_middlewares;

use crate::common::{AxumEmployee, AxumMasterData, AxumStudent};

lazy_static!{
    pub static ref STUDENT_DATA: Arc<RwLock<Vec<AxumStudent>>> = {
        let data = fs::read_to_string("json_data/AxumData/StudentData.json")
        .expect("Failed to read the JSON file");

        let students: Vec<AxumStudent> = serde_json::from_str(&data).expect("Unable to get data");
        Arc::new(RwLock::new(students))
    };

    pub static ref EMPLOYEE_DATA: Arc<RwLock<Vec<AxumEmployee>>> = {
        let data = fs::read_to_string("json_data/AxumData/EmployeeData.json")
        .expect("Failed to read the JSON file");

        let employees: Vec<AxumEmployee> = serde_json::from_str(&data).expect("Unable to get data");
        Arc::new(RwLock::new(employees))
    };

    pub static ref EXECUTIVE_DATA: Arc<RwLock<Vec<AxumMasterData>>> = {
        let data = fs::read_to_string("json_data/AxumData/ExecutivesData.json")
        .expect("Failed to read the JSON file");

        let executives: Vec<AxumMasterData> = serde_json::from_str(&data).expect("Unable to get data");
        Arc::new(RwLock::new(executives))
    };
}

///this is the starting point for axum_task
pub async fn axum_task_main() {
    let app = get_routes();
    let app = get_middlewares(app);
    println!("Server Running!!");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}