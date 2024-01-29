use serde_json::{json, Value};

use crate::common::Student;
use std::{collections::HashMap, fs};

///Function to calculate total marks,percentage and assign the grade
pub fn calculate_percentage_and_grade(student: &mut Student) {
    let total_marks: u32 = student.marks.iter().sum();
    let percentage = total_marks as f64 / student.marks.len() as f64;
    let grade = match percentage {
        _ if percentage >= 90.0 => "A+",
        _ if percentage >= 80.0 => "A",
        _ if percentage >= 70.0 => "B",
        _ if percentage >= 60.0 => "C",
        _ if percentage >= 50.0 => "D",
        _ => "F",
    };

    student.total = Some(total_marks);
    student.percentage = Some(percentage);  
    student.grade = Some(grade.to_string());
}

///Function to deserialize the data perform some operations and again serialize the data and store in differnt file using hashmap
pub fn student_with_hashmap_main() {
    let path = "json_data/StudentData/StudentData.json";

    let content = fs::read_to_string(path).expect("Failed to read file");

    //Deserialization
    let mut students: Vec<Student> = serde_json::from_str(&content).expect("Failed to parse JSON");

    for student in &mut students {
        calculate_percentage_and_grade(student);
    }

    let mut student_vec: Vec<HashMap<&str, Value>>  = Vec::new();

    for student in students{
        let mut student_hashmap: HashMap<&str, Value> = HashMap::new();
        student_hashmap.insert("name:", Value::String(student.name.to_string()));
        student_hashmap.insert("phone:", Value::String(student.phone.to_string()));
        student_hashmap.insert("email:", Value::String(student.email.to_string()));
        student_hashmap.insert("city:", Value::String(student.city.to_string()));
        student_hashmap.insert("address:", Value::String(student.address.to_string()));
        student_hashmap.insert("marks:", json!(student.marks));
        student_hashmap.insert("total:", json!(student.total));
        student_hashmap.insert("percentage:", json!(student.percentage));
        student_hashmap.insert("grade:", json!(student.grade));
        student_vec.push(student_hashmap);
    }
    
    // Serialization
    let updated_hashmap_json = serde_json::to_string_pretty(&student_vec).expect("Failed to serialize JSON");

    fs::write("json_data/StudentData/student_hashmap/updated_hashmap_json.json",updated_hashmap_json).expect("Failed to write file");
}

