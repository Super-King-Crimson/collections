use crate::HashMap;
pub mod ownership;
pub mod updating;

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

    //Get a value out with get
    let blue_score = scores.get("Blue").copied().unwrap_or(0);
    //Returns Option<&V>, copied() turns that into an Option<V> thru a clone, unwrap_or(0) sets to 0 if None

    //Iterate with a for loop (remember, be a good rustacean, don't take ownership)
    for (k, v) in &scores {
        println!("{k}: {v}");
    }

    ownership::explain();
    updating::explain();

    //Rust uses the SipHash hashing function,
    //which exchanges some speed for resistance against DoS (Denial of Service) attacks involving hash tables
    //You can switch hashers by making one that implements the BuildHasher trait (one day we'll learn them i swear)

    //Check the docs for more methods (like retain and remove_entry)
}