use std::{fs, sync::{Arc, RwLock}};

use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, Clone)]
pub struct GRPCExecutive {
    pub id: i32,
    pub name: String,
    pub skills: Vec<String>,
    pub status: String,
    pub language: String,
}

lazy_static!{
    #[derive(Deserialize,Serialize,Clone)]
    pub static ref EXECUTIVE_DATA: Arc<RwLock<Vec<GRPCExecutive>>> = {
        let data = fs::read_to_string("json_data/ExecutivesData.json")
        .expect("Failed to read the JSON file");

        let executive: Vec<GRPCExecutive> = serde_json::from_str(&data).expect("Unable to get data");
        Arc::new(RwLock::new(executive))
    };
}