use super::{RequestData, HASH_DATA};
use chrono::Utc;
use std::collections::{HashMap, VecDeque};

///this function bifurcates the task in different levels depending on the time
pub fn level_bifurcation() {
    let mut hash_temp: HashMap<Option<String>, VecDeque<RequestData>> = HashMap::new();

    for (each_key, each_data) in HASH_DATA.write().unwrap().iter_mut() {
        let mut new_key: Option<String> = None;
        let mut list_req = VecDeque::new();
        
        while let Some(each_req) = each_data.pop_front() {
            let current_time = Utc::now();
            let time_second_diff = current_time
            .signed_duration_since(each_req.time)
            .num_seconds();
        let mut level = 1;

            match time_second_diff {
                0..=29 => level = 1,
                30..=59 => level = 2,
                60..=89 => level = 3,
                90..=119 => level = 4,
                _ => level = 5,
            }

            if level > 1 {
                let mut old_queue_name: Vec<String> = each_key.split("_").map(|s| s.to_string()).collect();
                let queue_len = old_queue_name.len();
                old_queue_name[queue_len - 1] = format!("L{}", level);
                let new_queue_name = old_queue_name.join("_");
                println!("{}--> {}", each_key, &new_queue_name);
                new_key = Some(new_queue_name);
                list_req.push_back(each_req);
            }
        }

        hash_temp.entry(new_key)
            .and_modify(|queue| queue.append(&mut list_req))
            .or_insert(list_req);
    }

    for (each_key, mut each_req) in hash_temp {
        if each_key.is_some() {
            HASH_DATA
                .write()
                .expect("Unable to write")
                .entry(each_key.unwrap())
                .and_modify(|queue| {
                    queue.append(&mut each_req);
                });
        }
    }

    println!("level bifercation done!");
    println!();
}