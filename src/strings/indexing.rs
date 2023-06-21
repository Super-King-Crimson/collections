mod slicing_strings;

pub fn explain() {
    println!("It's pretty easy to get a character out of a string, just index it and-");
    let naive: String = String::from("ABC");
    // println!("{}", naive[0]);

    println!("WHAAAAAAAAAAAAAAAAAAAAAT");
    calm_down();
}

fn calm_down() {
    //Strings don't support indexing. Why? Because strings are actually really complex.
    //At it's core, a String is a wrapper over Vec<u8>, where each u8 is the byte value of a UTF-8 encoded char

    //The size of this vec, quite expectedly, is 2 bytes (one byte for each character)
    let hi = String::from("hi");

    //The size of this vec, however, is 6 bytes: each unicode scalar value in it is 3 bytes
    let hi_jp = String::from("よう");


    //So what would hi_jp[0] return? It wouldn't be useful to return the byte value, it's just a number
    for b in hi_jp.bytes() {
        print!("{b}, ");
    }
    println!();

    //It could return the first unicode scalar value (char)
    for c in hi_jp.chars() {
        print!("{c}, ");
    }
    println!();

    //But in Hindi, there are chars that actually don't represent a full letter (see the 4th and 6th char)
    let hindi = String::from("नमस्ते");
    for c in hindi.chars() {
        print!("{c}, ");
    }
    println!();
    //It wouldn't make sense to ever return these alone - they only mean smthn in the context of other chars
    //If it had to go through a string to check which chars are returnable, char indexes wouldn't be O(1)

    //So Rust doesn't make a decision - it leaves it up to the programmer
    slicing_strings::explain();
}