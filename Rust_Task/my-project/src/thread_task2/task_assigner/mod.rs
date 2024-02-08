use crate::thread_task2::{HASH_DATA, USER_DATA};


pub fn task_assigner() {
    let user_data = USER_DATA.read().expect("Unable to read");
    let mut hash_data = HASH_DATA.write().expect("Unable to write");

    for level in (1..=5).rev() {
        let level_key = format!("_\"L{}\"", level);
        for (key, data) in hash_data.iter_mut() {
            if key.ends_with(&level_key) {
                let mut matched_indexes = Vec::new();
                if !data.is_empty(){
                    for (index, request_data) in data.iter().enumerate() {
                        let mut matched = false;
                        for user in user_data.iter() {
                            if ((key.contains("Call") && user.status == "Offline")
                            || (key.contains("Chat") && user.status == "Online")) && user.skills.contains(&request_data.skill)
                                && user.language == request_data.language
                            {
                                println!("Matched data for user ID {}: {:?}", user.id, request_data);
                                matched = true;
                                break;
                            }
                        }

                        if matched {
                            matched_indexes.push(index);
                        }
                    }
                }
                matched_indexes.reverse();
                for index in matched_indexes {
                    data.remove(index);
                }
            }
        }
    }
    println!("task assigned!");
    println!();
}




// use crate::thread_task2::{HASH_DATA, USER_DATA};

// pub fn task_assigner() {
//     let user_data = USER_DATA.read().expect("Unable to read user data");
//     let mut hash_data = HASH_DATA.write().expect("Unable to write to hash data");

//     for level in 1..=5 {
//         let level_key = format!("_L{}", level);

//         for (key, data) in hash_data.iter_mut() {
//             if key.ends_with(&level_key) {
//                 let mut matched_indexes = Vec::new();

//                 for (index, request_data) in data.iter().enumerate() {
//                     let mut matched = false;

//                     for user in user_data.iter() {
//                         if user.skills.contains(&request_data.skill)
//                             && user.language == request_data.language
//                             && ((key.contains("Call") && user.status == "Offline")
//                                 || (key.contains("Chat") && user.status == "Online"))
//                         {
//                             println!("Matched data for user ID {}: {:?}", user.id, request_data);
//                             matched = true;
//                             break;
//                         }
//                     }

//                     if matched {
//                         matched_indexes.push(index);
//                     }
//                 }

//                 matched_indexes.reverse();
//                 for index in matched_indexes {
//                     data.remove(index);
//                 }
//             }
//         }
//     }

//     println!("Task assignment completed!");
//     println!();
// }
