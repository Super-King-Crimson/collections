pub mod updating;
pub mod indexing;

pub fn explain() {
    println!("We'll be talking about:");
    println!("  - How string is different from other collections");
    println!("  - Some methods to manipulate strings");
    println!("  - Why strings are actually kinda complex");

    elaborate();
}


fn elaborate() {
    //There is only one string type in the core Rust lang: str (or &str) for UTF-8 string slices stored somewhere
    //The String type is in std - growable, mutable, owned UTF-8 encoded string

    //Make an empty String with ::new (just like a vec)
    let mut ting = String::new();

    //Load data into a String with the to_string method on anything that implements the Display trait
    let mut another_ting = "Hello World".to_string();

    //Create a String directly from a string literal with String::from()
    let fring = String::from("Hi Earth");

    //They can also store different languages because of UTF-8 encoding
    updating::explain();
    indexing::explain();

    //As always, review the docs more more methods (like contains and replace)!
}