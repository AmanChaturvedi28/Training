use crate::common::Data;
use rand::Rng;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use chrono::Utc;
use chrono::DateTime;

///function to generate raname name of 5 character
fn get_name() -> String{
    let mut name = String::new();
    for i in 1..=5{
        let x = rand::thread_rng().gen_range('a'..='z');
        name.push(x);
    }
    name
}

pub fn thread_task1_main() {
    let v = RwLock::new(Vec::new());
    let arc = Arc::new(v);

    let ref1 = Arc::clone(&arc);
    let ref2 = Arc::clone(&arc);
    let ref3 = Arc::clone(&arc);

    let t1 = thread::spawn(move || loop{
        thread::sleep(Duration::from_secs(5));

        let my_name = get_name();
        let my_id = rand::thread_rng().gen_range(1..=100);
        let my_timestamp = Utc::now();
        
        println!("Entering Data: id: {:?}, name: {:?}, time: {:?}",my_id,my_name.as_str(),my_timestamp);
        ref1.write().expect("Unable to write data").push(Data::data_assign(my_id, my_name, my_timestamp));
        println!("");
    });

    let t2 = thread::spawn(move || loop{
        thread::sleep(Duration::from_secs(30));
            println!("Reading Data...");
            println!("Data: {:?}",ref2.read().expect("Unable to read data"));
            println!("");
    });

    let t3 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(60));

        let mut data = ref3.write().expect("Unable to write for t3");

        data.retain(|x| {
            let current_time = Utc::now();
            let duration = current_time.signed_duration_since(x.timestamp);
            duration.num_minutes() >= 1
        });

        println!("Data removed!");
        println!("");
    });
    
    t1.join().expect("Unable to join t1");
    t2.join().expect("Unable tot join t2");
    t3.join().expect("Unable to join t3");
}