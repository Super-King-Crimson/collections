use crate::HashMap;

pub fn explain() {
    println!("Hashmaps, like math functions, map a singular key to a singular value.");
    println!("When you change a key's value, you need to know how to handle the value already existing.");

    //The default option is overwriting the current value
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);    // {Blue: 10}
    scores.insert(String::from("Yellow"), 10);  // {Blue: 10, Yellow: 10}
    scores.insert(String::from("Yellow"), 25);  // {Blue: 10, Yellow: 25}, overwritten

    //You can also decide to only insert a value in if it doesn't already exist with the entry() method
    //This returns an Entry enum, 
        //which has the method or_insert(), which assigns its value to that key if it doesn't exist
    scores.entry(String::from("Green")).or_insert(50); //{Blue: 10, Yellow: 25, Red: 50}
    scores.entry(String::from("Blue")).or_insert(50);  //{Blue: 10, Yellow: 25, Red: 50}

    //Finally, you can decide to change the value assigned to the key based on the old one (also using entry)
    let no_way: String = String::from("the quick brown fox jumped over the lazy dog");
    let mut verifier: HashMap<char, i32> = HashMap::new();

    //Remember when we said read the docs to find cool methods? Read the docs.
    for c in no_way.split_whitespace().fold(String::new(), |acc, x| acc + x).chars() {
        let count = verifier.entry(c).or_insert(0);
        *count += 1;
    }
    println!("{verifier:?}");
}