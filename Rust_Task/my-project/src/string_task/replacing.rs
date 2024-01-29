use std::io;

/// Function to read a string and a character from the user to replace
pub fn replace_main() -> String{
    println!("Enter a string:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    println!("Enter a character to remove:");
    let mut input_char = String::new();
    io::stdin().read_line(&mut input_char).expect("Failed to read line");

    let result_string = remove_character(&input_string.to_lowercase(), &input_char);

    println!("Result: {}", result_string);
    result_string
}

/// Function to replace perticular character
pub fn remove_character(input: &str, char_to_remove: &str) -> String {
    let mut result = String::new();
    for i in input.chars() {
        if i.to_string() != char_to_remove.trim() {
            result.push(i);
        }
        else {
            result.push('_');
        }
    }
    result
}