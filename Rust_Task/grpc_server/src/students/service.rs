use super::{
    model::{GRPCStudent, STUDENT_DATA},
    student::StudentData,
};

pub async fn read_student(id: i32) -> Option<String> {
    let students = STUDENT_DATA.read().unwrap();
    let student = students.iter().find(|&s| s.id == id);
    match student {
        Some(student) => Some(serde_json::to_string(student).unwrap()),
        None => None,
    }
}

pub async fn read_all_student() -> Option<String> {
    let students = STUDENT_DATA.read().unwrap();
    if students.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&*students).unwrap())
    }
}

pub async fn delete_student(id: i32) -> Option<String> {
    let mut students = STUDENT_DATA.write().unwrap();
    if let Some(index) = students.iter().position(|s| s.id == id) {
        let deleted_student = students.remove(index);
        Some(serde_json::to_string(&deleted_student).unwrap())
    } else {
        None
    }
}

pub async fn update_student(updated_data: StudentData) -> Result<String, String> {
    let mut students = STUDENT_DATA.write().unwrap();

    // Check if the student exists
    if let Some(index) = students.iter().position(|s| s.id == updated_data.id) {
        // Update the student's information
        students[index].name = updated_data.name;
        students[index].phone = updated_data.phone;
        students[index].email = updated_data.email;
        students[index].city = updated_data.city;
        students[index].address = updated_data.address;
        students[index].marks = updated_data.marks;

        Ok(serde_json::to_string(&students[index]).unwrap())
    } else {
        if students.iter().any(|student| student.id == updated_data.id) {
            return Err("Student with the same ID already exists".to_string());
        }

        let grpc_student = map_structure(updated_data);

        let str_data = serde_json::to_string(&grpc_student).unwrap();

        students.push(grpc_student);

        Ok(str_data)
    }
}

pub async fn create_student(data: StudentData) -> Result<String, String> {
    let mut stud_data = STUDENT_DATA.write().unwrap();

    if stud_data.iter().any(|student| student.id == data.id) {
        return Err("Student with the same ID already exists".to_string());
    }

    let grpc_student = map_structure(data);

    let str_data = serde_json::to_string(&grpc_student).unwrap();

    stud_data.push(grpc_student);

    Ok(str_data)
}

fn map_structure(data: StudentData) -> GRPCStudent {
    let data = GRPCStudent {
        id: data.id,
        name: data.name,
        phone: data.phone,
        email: data.email,
        city: data.city,
        address: data.address,
        marks: data.marks,
    };
    data
}
