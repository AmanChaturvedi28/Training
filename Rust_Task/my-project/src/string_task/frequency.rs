use std::io;
use crate::string_task::replacing::replace_main;

/// Function to read two strings from user, separate characters based on frequency and replace underscore(_) in replace_main function
pub fn frequency_main() {
    println!("Enter the first string:");
    let mut string1 = String::new();
    io::stdin().read_line(&mut string1).expect("Failed to read line");

    println!("Enter the second string:");
    let mut string2 = String::new();
    io::stdin().read_line(&mut string2).expect("Failed to read line");

    let frequency1 = calculate_frequency(&string1.to_lowercase());
    let frequency2 = calculate_frequency(&string2.to_lowercase());

    let mut vector1 = Vec::new();
    let mut vector2 = Vec::new();
    let mut final_vector = Vec::new();

    for ch in 'a'..='z' {
        let index = ch as usize - 'a' as usize;

        let count1 = frequency1[index];
        let count2 = frequency2[index];

        if count1 > 0 && count2 > 0 {
            vector1.push((ch, count1 + count2));
            final_vector.push((ch, count1 + count2));
        } else if count1 > 0 {
            vector2.push((ch, count1));
            final_vector.push((ch, count1));
        } else if count2 > 0 {
            vector2.push((ch, count2));
            final_vector.push((ch, count2));
        }
    }

    println!("Vector 1 (Common characters with total counts in both strings): {:?}", vector1);
    println!("Vector 2 (Characters not common in both strings): {:?}", vector2);
    println!("Final List: {:?}", final_vector);

    let replacing_char = replace_main();

    for i in replacing_char.chars() {
        if i == '_' {
            if let Some((ch, count)) = vector1.first_mut() {
                print!("{}", *ch);
    
                if *count > 0 {
                    *count -= 1;
    
                    if *count == 0 {
                        vector1.remove(0);
                    }
                }
            } else {
                print!("_");
            }
        } else {
            print!("{}", i);
        }
    }
}

///function to calculate frequency of characters
pub fn calculate_frequency(s: &str) -> [usize; 26] {
    let mut frequency = [0; 26];
    for ch in s.chars() {
        if ch >= 'a' && ch <= 'z' {
            let index = ch as usize - 'a' as usize;
            frequency[index] += 1;
        }   
    }
    frequency
}  