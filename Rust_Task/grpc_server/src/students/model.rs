use std::{fs, sync::{Arc, RwLock}};

use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct GRPCStudent {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<u32>,
}

lazy_static!{
    pub static ref STUDENT_DATA: Arc<RwLock<Vec<GRPCStudent>>> = {
        let data = fs::read_to_string("json_data/StudentData.json")
        .expect("Failed to read the JSON file");

        let students: Vec<GRPCStudent> = serde_json::from_str(&data).expect("Unable to get data");
        Arc::new(RwLock::new(students))
    };
}