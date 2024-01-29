use crate::common::{Employee, Position, Skill};
use std::fs;

///Function to deserialize the data perform some operations and again serialize the data and store in differnt files
pub fn employee_main() {
    let content = fs::read_to_string("json_data/EmployeeData/Employee.json").expect("File not found");

    //Deserialization
    let employees: Vec<Employee> = serde_json::from_str(&content).expect("Unable to get data");

    let mut software_developer_data: Vec<Employee> = Vec::new();
    let mut jr_developer_data: Vec<Employee> = Vec::new();
    let mut sr_developer_data: Vec<Employee> = Vec::new();
    let mut other_data: Vec<Employee> = Vec::new();

    for employee in employees {
        if employee.position == Some(Position::JrSoftwareDeveloper) && employee.skills.contains(&Skill::Java) {
            jr_developer_data.push(employee);
        }
        else if employee.position == Some(Position::SoftwareDeveloper) && employee.skills.contains(&Skill::Rust) {
            software_developer_data.push(employee);
        }
        else if employee.position == Some(Position::SrSoftwareDeveloper) || employee.skills.contains(&Skill::CSharp){
            sr_developer_data.push(employee);
        }
        else {
            other_data.push(employee);
        }      
    }
    
    // Serialization
    let mid_emp_data = serde_json::to_string_pretty(&software_developer_data)
    .expect("Unable to convert JSON");
    fs::write("json_data/EmployeeData/employee_structure/mid_emp.json", mid_emp_data).expect("unable to create file");
    
    // Serialization
    let jr_emp_data = serde_json::to_string_pretty(&jr_developer_data)
    .expect("Unable to convert JSON");
    fs::write("json_data/EmployeeData/employee_structure/jr_emp.json", jr_emp_data).expect("unable to create file");

    // Serialization
    let sr_emp_data = serde_json::to_string_pretty(&sr_developer_data)
    .expect("Unable to convert JSON");
    fs::write("json_data/EmployeeData/employee_structure/sr_emp.json", sr_emp_data).expect("unable to create file");
    
    // Serialization
    let emp_data = serde_json::to_string_pretty(&other_data).expect("Unable to convert JSON");
    fs::write("json_data/EmployeeData/employee_structure/other_emp.json", emp_data).expect("unable to create file");
}
