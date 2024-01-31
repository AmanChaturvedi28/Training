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

// use crate::student::student_main;
use self::employee::employee_main;
use crate::string_task::{frequency::frequency_main,replacing::replace_main};
use crate::table::table_main;
use  self::employee_with_hashmap::employee_with_hashmap_main;
use  self::student_with_hashmap::student_with_hashmap_main;
use crate::table::table_main;

///This is the entry point for this application. It calls various functions from different modules
pub fn main() {  
    // student_main();
    // student::student_main();
    // student_with_hashmap_main();
    // employee_main();
    // employee_with_hashmap_main();
    // frequency_main();
    // replace_main();
    // table_main();
    new_table();
}