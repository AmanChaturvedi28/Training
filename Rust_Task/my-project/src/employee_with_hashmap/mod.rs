use std::{collections::HashMap, fs};

use serde_json::{json, Value};

use crate::common::{Employee, Position, Skill};

///Function to deserialize the data perform some operations and again serialize the data and store in differnt files using hashmap
pub fn employee_with_hashmap_main() {
    let content = fs::read_to_string("json_data/EmployeeData/Employee.json").expect("File not found");

    //Deserialization
    let employees: Vec<Employee> = serde_json::from_str(&content).expect("Unable to get data");
    // let mut employees: HashMap<&str, serde_json::Value> = serde_json::from_str(&content).expect("Unable to get data");
    
    let mut software_developer_hashmap: Vec<HashMap<&str, Value>> = Vec::new();
    let mut jr_developer_hashmap: Vec<HashMap<&str, Value>> = Vec::new();
    let mut sr_developer_hashmap: Vec<HashMap<&str, Value>> = Vec::new();
    let mut other_hashmap: Vec<HashMap<&str, Value>> = Vec::new();

    for employee in employees {
        if employee.position == Some(Position::JrSoftwareDeveloper) && employee.skills.contains(&Skill::Java) {
            let mut jr_data: HashMap<&str, Value> = HashMap::new();
            jr_data.insert("name:",Value::String(employee.name));
            jr_data.insert("age:",Value::String(employee.age.to_string()));
            jr_data.insert("skills:",json!(employee.skills));
            jr_data.insert("position:",json!(employee.position));
            jr_data.insert("experience:",json!(employee.experience));
            jr_developer_hashmap.push(jr_data);
        }
        else if employee.position == Some(Position::SoftwareDeveloper) && employee.skills.contains(&Skill::Rust) {
            let mut mid_data: HashMap<&str, Value> = HashMap::new();
            mid_data.insert("name:",Value::String(employee.name));
            mid_data.insert("age:",Value::String(employee.age.to_string()));
            mid_data.insert("skills:",json!(employee.skills));
            mid_data.insert("position:",json!(employee.position));
            mid_data.insert("experience:",json!(employee.experience));
            software_developer_hashmap.push(mid_data);
        }
        else if employee.position == Some(Position::SrSoftwareDeveloper) || employee.skills.contains(&Skill::CSharp){
            let mut sr_data: HashMap<&str, Value> = HashMap::new();
            sr_data.insert("name:",Value::String(employee.name));
            sr_data.insert("age:",Value::String(employee.age.to_string()));
            sr_data.insert("skills:",json!(employee.skills));
            sr_data.insert("position:",json!(employee.position));
            sr_data.insert("experience:",json!(employee.experience));
            sr_developer_hashmap.push(sr_data);
        }
        else {
            let mut other_data: HashMap<&str, Value> = HashMap::new();
            other_data.insert("name:",Value::String(employee.name));
            other_data.insert("age:",Value::String(employee.age.to_string()));
            other_data.insert("skills:",json!(employee.skills));
            other_data.insert("position:",json!(employee.position));
            other_data.insert("experience:",json!(employee.experience));
            other_hashmap.push(other_data);
        }      
    }
    
    
    // Serialization
    let mid_emp_hashmap = serde_json::to_string_pretty(&software_developer_hashmap)
    .expect("Unable to convert JSON");
    fs::write("json_data/EmployeeData/employee_hashmap/mid_emp_hashmap.json", mid_emp_hashmap).expect("unable to create file");
    
    // Serialization
    let jr_emp_hashmap = serde_json::to_string_pretty(&jr_developer_hashmap)
    .expect("Unable to convert JSON");
    fs::write("json_data/EmployeeData/employee_hashmap/jr_emp_hashmap.json", jr_emp_hashmap).expect("unable to create file");

    // Serialization
    let sr_emp_hashmap = serde_json::to_string_pretty(&sr_developer_hashmap)
    .expect("Unable to convert JSON");
    fs::write("json_data/EmployeeData/employee_hashmap/sr_emp_hashmap.json", sr_emp_hashmap).expect("unable to create file");
    
    // Serialization
    let other_emp_hashmap = serde_json::to_string_pretty(&other_hashmap).expect("Unable to convert JSON");
    fs::write("json_data/EmployeeData/employee_hashmap/other_emp_hashmap.json", other_emp_hashmap).expect("unable to create file");
}
