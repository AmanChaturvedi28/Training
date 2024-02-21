use std::{fs, sync::{Arc, RwLock}};

use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GRPCEmployee {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub skills: Vec<String>,
    pub position: Option<String>,
    pub experience: Option<i32>,
}

lazy_static!{
    pub static ref EMPLOYEE_DATA: Arc<RwLock<Vec<GRPCEmployee>>> = {
        let data = fs::read_to_string("json_data/EmployeeData.json")
        .expect("Failed to read the JSON file");

        let employees: Vec<GRPCEmployee> = serde_json::from_str(&data).expect("Unable to get data");
        Arc::new(RwLock::new(employees))
    };
}