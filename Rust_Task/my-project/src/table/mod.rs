use std::io;
use crate::common::{Cell,Row,Table};

/// Main function for demonstrating the table functionality.
pub fn table_main() {
    let cell1 = Cell::data_assign(1, 2, 3);
    let cell2 = Cell::data_assign(2, 3, 4);
    let row1 = Row::row_data(vec![cell1, cell2]);

    let cell3 = Cell::data_assign(3, 2, 5);
    let cell4 = Cell::data_assign(4, 3, 6);
    let row2 = Row::row_data(vec![cell3, cell4]);

    let mut table = Table::table_data(vec![row1, row2]);
    println!("{:#?}", table);

    println!("do you want to update table: (y/n) ");
    let mut update_input = String::new();
    io::stdin().read_line(&mut update_input).expect("Failed to read line");

    if update_input == "y"{
        let update_row = get_user_input("Enter the row index to be updated: ");
        let update_cell = get_user_input("Enter the cell index to be updated: ");
        let update_height: u32 = get_user_input("Enter the new height: ") as u32;
        let update_width: u32 = get_user_input("Enter the new width: ") as u32;
        let update_value: u32 = get_user_input("Enter the new value: ") as u32;

        table.rows[update_row].cells[update_cell] = Cell::data_assign(update_height, update_width, update_value);
        table.rows[update_row].update_row_data();
        table.update_table_data();
        println!("{:#?}", table);
    }
    else {
        println!("Thank you");
    }
}

/// Helper function to get user input as usize.
pub fn get_user_input(prompt: &str) -> usize {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Please enter a valid number.");
            0
        }
    }
}