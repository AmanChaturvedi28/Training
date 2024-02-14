use super::{SKILLS, USER_DATA};
use rand::{thread_rng, Rng};

///this function shuffles all data given
pub fn shuffle_data() {
    let mut ref1 = USER_DATA.write().expect("Unable to write");

    let person_skills = &SKILLS;
    let person_language = ["English", "Spanish"];
    let status = ["Online", "Offline"];

    for user in ref1.iter_mut() {
        user.skills.clear();
        user.language.clear();
        user.status.clear();

        let random_language_index = thread_rng().gen_range(0..=1);
        user.language = person_language[random_language_index].to_string();

        let random_status_index = thread_rng().gen_range(0..=1);
        user.status = status[random_status_index].to_string();

        let mut rng = thread_rng();
        let num_skills = thread_rng().gen_range(2..=3);
        for _ in 0..num_skills {
            let skill_index = rng.gen_range(0..person_skills.len());
            let new_skill = person_skills[skill_index].to_string();
            user.skills.push(new_skill);

            // if !user.skills.contains(&new_skill) {
            // user.skills.push(new_skill);
            // } else {
            //     let mut new_skill_index = rng.gen_range(0..person_skills.len());
            //     while user.skills.contains(&person_skills[new_skill_index]) {
            //         new_skill_index = rng.gen_range(0..person_skills.len());
            //     }
            //     user.skills.push(person_skills[new_skill_index].to_string());
            // }
        }
    }
    // println!("{:?}",ref1);
    println!("shuffling data!");
    println!();
}
