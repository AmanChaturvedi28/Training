///common module includes all the structures and enums for the project
pub mod common;
///student module includes functions for student data
pub mod student;
///employee module includes functions for employee data
pub mod employee;
///string_task module includes functions for string_task
pub mod string_task;
///table module includes functions for table data
pub mod table;
///employee module includes functions for employee data which uses hashmap
pub mod employee_with_hashmap;
///student module includes functions for employee data which uses hashmap
pub mod student_with_hashmap;
///table module includes functions for formatting the table based on the data
pub mod new_table;
///thread_task1 module includes functions for adding, reading and deleting data on different thread
pub mod thread_task1;
///thread_task2 module includes functions for task bifurcation and task assigner
pub mod thread_task2;
///axum_task include function for creating server and routes for different requests
pub mod axum_task;
pub mod area;

// use crate::student::student_main;
// use self::employee::employee_main;
// use crate::string_task::{frequency::frequency_main,replacing::replace_main};
// use crate::table::table_main;
// use  self::employee_with_hashmap::employee_with_hashmap_main;
// use  self::student_with_hashmap::student_with_hashmap_main;
// use crate::thread_task1::thread_task1_main;
// use crate::thread_task2::thread_task2_main;
// use self::axum_task::axum_task_main;
use area::area_main;

///This is the entry point for this application. It calls various functions from different modules
// #[tokio::main]
pub fn main() {  
    // student_main();
    // student::student_main();
    // student_with_hashmap_main();
    // employee_main();
    // employee_with_hashmap_main();
    // frequency_main();
    // replace_main();
    // table_main();
    // new_table();
    // thread_task1_main();
    // thread_task2_main();
    // axum_task_main().await;
    area_main();
}