use crate::{
    axum_task::EMPLOYEE_DATA,
    common::{AxumEmployee, Message},
};
use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};

pub async fn read_employee_id(Path(id): Path<i32>) -> Response {
    let employee_id = id;
    let employee_data = EMPLOYEE_DATA.read().unwrap();

    if let Some(employee) = employee_data
        .iter()
        .find(|employee| employee.id == employee_id)
    {
        return Json(Message {
            status: 2000,
            message_key: String::from("success!!"),
            data: employee.clone(),
        })
        .into_response();
    } else {
        return Json(Message {
            status: 4004,
            message_key: String::from("failed!!"),
            data: String::from("Employee with this id does not exist"),
        })
        .into_response();
    }
}

pub async fn read_employee() -> Response {
    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: EMPLOYEE_DATA.read().unwrap().clone(),
        })
    }
    .into_response();
}

pub async fn delete_employee(Path(id): Path<i32>) -> Response {
    let employee_id = id;
    let mut employee_data = EMPLOYEE_DATA.write().unwrap();

    if let Some(index) = employee_data
        .iter()
        .position(|employee| employee.id == employee_id)
    {
        let removed_employee = employee_data.remove(index);
        return Json(Message {
            status: 2000,
            message_key: String::from("success: employee deleted"),
            data: removed_employee,
        })
        .into_response();
    } else {
        return Json(Message {
            status: 4004,
            message_key: String::from("error"),
            data: String::from("employee with this id does not exist"),
        })
        .into_response();
    }
}

pub async fn update_employee(Json(update_employee): Json<AxumEmployee>) -> Response {
    let employee_id = update_employee.id;
    let mut employee_data = EMPLOYEE_DATA.write().unwrap();
    if let Some(employee) = employee_data
        .iter_mut()
        .find(|employee| employee.id == employee_id)
    {
        employee.name = update_employee.name;
        employee.age = update_employee.age;
        employee.position = update_employee.position;
        employee.experience = update_employee.experience;
        employee.skills = update_employee.skills;
        return Json(Message {
            status: 2000,
            message_key: String::from("success: employee updated"),
            data: employee.clone(),
        })
        .into_response();
    } else {
        employee_data.push(update_employee.clone());
        return Json(Message {
            status: 2000,
            message_key: String::from("success: new employee created"),
            data: update_employee.clone()
        })
        .into_response();
    }
}

pub async fn create_employee(Json(employee): Json<AxumEmployee>) -> Response {
    let employee_id = employee.id;

    let mut employee_data = EMPLOYEE_DATA.write().unwrap();
    if employee_data
        .iter()
        .any(|employee| employee.id == employee_id)
    {
        return Json(Message {
            status: 4004,
            message_key: String::from("failed!!"),
            data: String::from("employee with this id already exist"),
        })
        .into_response();
    } else {
        employee_data.push(employee.clone());
        return Json(Message {
            status: 2000,
            message_key: String::from("success!!"),
            data: employee.clone(),
        })
        .into_response();
    }
}
