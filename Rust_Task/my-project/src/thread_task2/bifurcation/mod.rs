use std::{collections::VecDeque, sync::{Arc, RwLock}};
use crate::thread_task2::RANDOM_DATA;
use super::{RequestData, HASH_DATA};

///this function bifurcates all the task for level1
pub fn bifurcation (){
    let bifer_ref: Arc<RwLock<VecDeque<RequestData>>> = Arc::clone(&RANDOM_DATA);

    if let Some(each_data) = bifer_ref.write().expect("Unable to write").pop_front(){
        let skill = &each_data.skill;
        let available = &each_data.available;
        let language = &each_data.language;
        let each_user = format!("{}_{:?}_{:?}_\"L1\"", skill, language, available);
        HASH_DATA.write().expect("Unable to write").entry(each_user).or_insert_with(VecDeque::new).push_front(each_data);
    };
    
    println!("bifurcation done!");
    println!();
}