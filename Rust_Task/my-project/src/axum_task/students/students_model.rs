use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};

use crate::{axum_task::STUDENT_DATA, common::{AxumStudent, Message}};

pub async fn read_student() -> Response {
    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: STUDENT_DATA.read().unwrap().clone(),
        })
    }
    .into_response();
}

pub async fn read_student_id(Path(id): Path<i32>) -> Response {
    let student_id = id;
    let student_data = STUDENT_DATA.read().unwrap();

    if let Some(student) = student_data.iter().find(|student| student.id == student_id) {
        return Json(Message {
            status: 2000,
            message_key: String::from("success!!"),
            data: student.clone(),
        })
        .into_response()
    }else {
        return Json(Message {
            status: 2000,
            message_key: String::from("failed!!"),
            data: String::from("student with this id does not exist"),
        })
        .into_response()
    }
}

pub async fn delete_student(Path(id): Path<i32>) -> Response {
    let student_id = id;
    let mut student_data = STUDENT_DATA.write().unwrap();
    if let Some(index) = student_data
        .iter()
        .position(|student| student.id == student_id)
    {
        let removed_student = student_data.remove(index);
        return Json(Message {
            status: 2000,
            message_key: String::from("success: student deleted"),
            data: removed_student,
        })
        .into_response();
    } else {
        return Json(Message {
            status: 4004,
            message_key: String::from("error"),
            data: String::from("Student with this id does not exist"),
        })
        .into_response();
    }
}

pub async fn update_student(Json(update_student): Json<AxumStudent>) -> Response {
    let student_id = update_student.id;
    let mut student_data = STUDENT_DATA.write().unwrap();
    if let Some(student) = student_data
        .iter_mut()
        .find(|student| student.id == student_id)
    {
        student.name = update_student.name;
        student.phone = update_student.phone;
        student.email = update_student.email;
        student.city = update_student.city;
        student.address = update_student.address;
        student.marks = update_student.marks;
        return Json(Message {
            status: 2000,
            message_key: String::from("success: executive updated"),
            data: student.clone(),
        })
        .into_response();
    } else {
        student_data.push(update_student.clone());
        return Json(Message {
            status: 2000,
            message_key: String::from("success: new executive created"),
            data: update_student.clone(),
        })
        .into_response();
    }
}

pub async fn create_student(Json(student): Json<AxumStudent>) -> Response {
    let student_id = student.id;

    let mut student_data = STUDENT_DATA.write().unwrap();
    if student_data
        .iter()
        .any(|student| student.id == student_id)
    {
        return Json(Message {
            status: 4004,
            message_key: String::from("failed!!"),
            data: String::from("student with this id already exist"),
        })
        .into_response();
    } else {
        student_data.push(student.clone());
        return Json(Message {
            status: 2000,
            message_key: String::from("success!!"),
            data: student.clone(),
        })
        .into_response();
    }
}