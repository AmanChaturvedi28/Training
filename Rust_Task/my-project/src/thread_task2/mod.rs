pub mod random_data;
use random_data::random_data;
pub mod bifurcation;
use bifurcation::bifurcation;
pub mod level_bifurcation;
use level_bifurcation::level_bifurcation;
pub mod shuffle_data;
use shuffle_data::shuffle_data;
pub mod task_assigner;
use task_assigner::task_assigner;

use std::{collections::{HashMap, VecDeque}, fs, sync::{Arc, RwLock}, thread::{self}, time::Duration};
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::Deserialize;
use crate::common::{MasterData,RequestData};

lazy_static! {
    pub static ref USER_DATA: Arc<RwLock<VecDeque<MasterData>>> = {
        let data = fs::read_to_string("json_data/ThreadData/Master_Data.json")
            .expect("Failed to read the JSON file");
        
        Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
    };

    pub static ref SKILLS: Vec<String> = vec![
        "Customer Service".to_string(),
        "Problem-solving".to_string(),
        "Product Knowledge".to_string(),
        "Effective Communication".to_string(),
        "Time Management".to_string(),
        "Adaptability".to_string(),
        "Team Collaboration".to_string(),
        "Feedback Analysis".to_string(),
        "Proactive Engagement".to_string(),
        "Technical Proficiency".to_string(),
        "Cultural Sensitivity".to_string(),
        "Documentation".to_string(),
    ];
    
    pub static ref HASH_DATA: Arc<RwLock<HashMap<String, VecDeque<RequestData>>>> = {
        let mut hash_map: HashMap<String, VecDeque<RequestData>> = HashMap::new();
        
        for skill in SKILLS.iter() {
            for language in &["English".to_string(), "Spanish".to_string()] {
                for available in &["Chat".to_string(), "Call".to_string()] {
                    for level in &["L1".to_string(), "L2".to_string(), "L3".to_string(), "L4".to_string(), "L5".to_string()] {
                        let key = format!("{}_{:?}_{:?}_{:?}", skill, language, available, level);
                        hash_map.insert(key.to_string(), VecDeque::new());
                        
                    }
                }
            }
        }
        Arc::new(RwLock::new(hash_map))
    };

    #[derive(Debug)]
    pub static ref RANDOM_DATA: Arc<RwLock<VecDeque<RequestData>>> = Arc::new(RwLock::new(VecDeque::new()));
}

///this function is the starting point for thread task2 which includes multiple thread working on different functionalities
pub fn thread_task2_main() {
    // let abc=Arc::clone(&HASH_DATA);
    let t1 = thread::spawn(move || loop{
        thread::sleep(Duration::from_secs(60));
        shuffle_data();
    });

    let t2 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        random_data();
    });

    let t3 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(3));
        bifurcation();
    });

    let t4 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        level_bifurcation();
    });

    let t5 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        task_assigner();
    });

    t1.join().expect("Failed to join thread 1");
    t2.join().expect("Failed to join thread 2");
    t3.join().expect("Failed to join thread 3");
    t4.join().expect("Failed to join thread 4");
    t5.join().expect("Failed to join thread 5");
}