use std::collections::HashMap;

#[derive(Debug)]
pub struct PlayerData {
    kills: u64,
    deaths: u64,
    level: u8,
    money: f64,
    friends: Vec<String>,
}
impl PlayerData {
    fn new() -> Self {
        PlayerData { kills: 0, deaths: 0, level: 1, money: 1000.0, friends: vec![String::from("1")]}
    }
}

pub fn explain() {
    println!("I wonder what Rust's hashing function looks like...");

    //Idle thoughts aside.
    //For stuff on the stack (implements Copy), values are copied into the hashmap
    //For stuff on the heap, values are owned by the hashmap
    let player_id: String = String::from("1959829158");
    let data: PlayerData = PlayerData::new();

    let mut john = HashMap::new();
    john.insert(player_id, data);
    // println!("{player_id}: {data:?}"); 

    //However, if we pass references into a hashmap, it won't take ownership
    //We can ensure that the values are valid for the entirety of the hashmap's lifetimes with reference lifetimes
}