use crate::{
    axum_task::EXECUTIVE_DATA,
    common::{AxumMasterData, Message},
};
use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};

///this handler reads all executives data
pub async fn read_executive() -> Response {
    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: EXECUTIVE_DATA.read().unwrap().clone(),
        })
    }
    .into_response();
}

///this handler reads the executive data based on id
pub async fn read_executive_id(Path(id): Path<i32>) -> Response {
    let executive_id = id;
    let executive_data = EXECUTIVE_DATA.read().unwrap();

    if let Some(executive) = executive_data
        .iter()
        .find(|executive| executive.id == executive_id)
    {
        return Json(Message {
            status: 200,
            message_key: String::from("success!!"),
            data: executive.clone(),
        })
        .into_response()
    }
    else {
        return Json(Message {
            status: 200,
            message_key: String::from("failed!!"),
            data: String::from("data with this id does not exist"),
        })
        .into_response()
    }
}

///this handler deletes the executive data for specified id
pub async fn delete_executive(Path(id): Path<i32>) -> Response {
    let executive_id = id;
    let mut executive_data = EXECUTIVE_DATA.write().unwrap();

    if let Some(index) = executive_data
        .iter()
        .position(|executive| executive.id == executive_id)
    {
        let removed_employee = executive_data.remove(index);
        return Json(Message {
            status: 2000,
            message_key: String::from("success: executive deleted"),
            data: removed_employee,
        })
        .into_response();
    } else {
        return Json(Message {
            status: 4004,
            message_key: String::from("error"),
            data: String::from("executive with this id does not exist"),
        })
        .into_response();
    }
}

///this handler updates the executive data 
pub async fn update_executive(Json(update_executive): Json<AxumMasterData>) -> Response {
    let executive_id = update_executive.id;
    let mut executive_data = EXECUTIVE_DATA.write().unwrap();
    if let Some(executive) = executive_data
        .iter_mut()
        .find(|employee| employee.id == executive_id)
    {
        executive.name = update_executive.name;
        executive.skills = update_executive.skills;
        executive.status = update_executive.status;
        executive.language = update_executive.language;
        return Json(Message {
            status: 2000,
            message_key: String::from("success: executive updated"),
            data: executive.clone(),
        })
        .into_response();
    } else {
        executive_data.push(update_executive.clone());
        return Json(Message {
            status: 2000,
            message_key: String::from("success: new executive created"),
            data: update_executive.clone(),
        })
        .into_response();
    }
}

///this function create the new executive
pub async fn create_executive(Json(executive): Json<AxumMasterData>) -> Response {
    let executive_id = executive.id;

    let mut executive_data = EXECUTIVE_DATA.write().unwrap();
    if executive_data
        .iter()
        .any(|executive| executive.id == executive_id)
    {
        return Json(Message {
            status: 4004,
            message_key: String::from("failed!!"),
            data: String::from("executive with this id already exist"),
        })
        .into_response();
    } else {
        executive_data.push(executive.clone());
        return Json(Message {
            status: 2000,
            message_key: String::from("success!!"),
            data: executive.clone(),
        })
        .into_response();
    }
}
