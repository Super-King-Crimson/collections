use std::collections::HashMap;

pub fn explain() {
    println!("Apparently hashmaps aren't as complex as strings.");

    //The official type is HashMap<K, V>, which maps keys of type K to values of type V.
    //It does this using a hashing function, which determines how it places them into memory.
        //It's like a lua dictionary!
    
    //Let's say we have two teams, and we want to store their scores, associating the team name with a score
     
    //Make empty HashMap with new (HashMap isn't part of the prelude: bring into scope with use)
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 2);
    scores.insert(String::from("Yellow"), -1);

    //Check the docs for more methods (like )
}