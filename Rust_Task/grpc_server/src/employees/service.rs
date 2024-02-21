use super::{
    employee::EmployeeData, model::{GRPCEmployee, EMPLOYEE_DATA}
};

pub async fn read_employee(id: i32) -> Option<String> {
    let employees = EMPLOYEE_DATA.read().unwrap();
    let employee = employees.iter().find(|&s| s.id == id);
    match employee {
        Some(employee) => Some(serde_json::to_string(employee).unwrap()),
        None => None,
    }
}

pub async fn read_all_employee() -> Option<String> {
    let employees = EMPLOYEE_DATA.read().unwrap();
    if employees.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&*employees).unwrap())
    }
}

pub async fn delete_employee(id: i32) -> Option<String> {
    let mut employees = EMPLOYEE_DATA.write().unwrap();
    if let Some(index) = employees.iter().position(|s| s.id == id) {
        let deleted_employee = employees.remove(index);
        Some(serde_json::to_string(&deleted_employee).unwrap())
    } else {
        None
    }
}

pub async fn create_employee(data: EmployeeData) -> Result<String, String> {
    let mut employees = EMPLOYEE_DATA.write().unwrap();

    if employees.iter().any(|employee| employee.id == data.id) {
        return Err("Employee with the same ID already exists".to_string());
    }

    let grpc_employee = map_structure(data);

    let str_data = serde_json::to_string(&grpc_employee).unwrap();

    employees.push(grpc_employee);

    Ok(str_data)
}

pub async fn update_employee(updated_data: EmployeeData) -> Result<String, String> {
    let mut employees = EMPLOYEE_DATA.write().unwrap();
    if let Some(index) = employees.iter().position(|s| s.id == updated_data.id) {
        employees[index].name = updated_data.name;
        employees[index].age = updated_data.age;
        employees[index].skills = updated_data.skills;
        employees[index].position = Some(updated_data.position);
        employees[index].experience = Some(updated_data.experience);

        Ok(serde_json::to_string(&employees[index]).unwrap())
    } else {
        if employees.iter().any(|employee| employee.id == updated_data.id) {
            return Err("Employee with the same ID already exists".to_string());
        }

        let grpc_employee = map_structure(updated_data);

        let str_data = serde_json::to_string(&grpc_employee).unwrap();

        employees.push(grpc_employee);

        Ok(str_data)
    }
}

fn map_structure(data: EmployeeData) -> GRPCEmployee {
    let data = GRPCEmployee {
        id: data.id,
        name: data.name,
        age: data.age,
        skills: data.skills,
        position: Some(data.position),
        experience: Some(data.experience)
    };
    data
}