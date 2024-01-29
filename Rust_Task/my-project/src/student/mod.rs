use crate::common::Student;
use std::fs;

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

///Function to deserialize the data perform some operations and again serialize the data and store in differnt file
pub fn student_main() {
    let path = "json_data/StudentData/StudentData.json";

    let content = fs::read_to_string(path).expect("Failed to read file");

    //Deserialization
    let mut students: Vec<Student> = serde_json::from_str(&content).expect("Failed to parse JSON");

    for student in &mut students {
        calculate_percentage_and_grade(student);
    }

    // Serialization
    let updated_json = serde_json::to_string_pretty(&students).expect("Failed to serialize JSON");

    fs::write("json_data/StudentData/student_structure/updated_json.json",updated_json).expect("Failed to write file");
}

