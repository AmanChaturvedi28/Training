use crate::thread_task2::{RANDOM_DATA, SKILLS};

use super::RequestData;
use chrono::{DateTime, Utc};
use rand::Rng;
use std::{collections::VecDeque, sync::{Arc, RwLock}, time::SystemTime};

pub fn random_data() {
    let ref1: Arc<RwLock<VecDeque<RequestData>>> = Arc::clone(&RANDOM_DATA);
    let ref2: Arc<RwLock<VecDeque<RequestData>>> = Arc::clone(&RANDOM_DATA);
    let skill = rand::thread_rng().gen_range(1..=12);
    let rnd_skill = match skill {
        1 => &SKILLS[0],
        2 => &SKILLS[1],
        3 => &SKILLS[2],
        4 => &SKILLS[3],
        5 => &SKILLS[4],
        6 => &SKILLS[5],
        7 => &SKILLS[6],
        8 => &SKILLS[7],
        9 => &SKILLS[8],
        10 => &SKILLS[9],
        11 => &SKILLS[10],
        12 => &SKILLS[11],
        _ => todo!(),
    };

    let language_index = rand::thread_rng().gen_range(0..2);
    let rnd_language = match language_index {
        0 => "English".to_string(),
        1 => "Spanish".to_string(),
        _ => panic!("Invalid language index"),
    };

    let current_time = SystemTime::now();

    let available_index = rand::thread_rng().gen_range(0..2);
    let rnd_available = match available_index {
        0 => "Chat".to_string(),
        1 => "Call".to_string(),
        _ => panic!("Invalid available index"),
    };

    let request_data = RequestData {
        skill: rnd_skill.to_string(),
        language: rnd_language,
        available: rnd_available,
        time: DateTime::<Utc>::from(current_time),
    };

    ref1.write().expect("Failed to write").push_back(request_data);
    println!("random Data generated!");
    println!();
}
