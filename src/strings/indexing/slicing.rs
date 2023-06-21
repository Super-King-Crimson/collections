pub fn explain() {
    //Since we don't know if a string index should return a byte, a char, or a string slice, you can't index strings.
    //Instead, use a range to contain a specific string's bytes
    let hi_jp = String::from("よう");
    let yo = &hi_jp[0..3]; //よ
    //Do know that if you ever slice out a part of a char's bytes ([0..1]) it would error

    //Additionally, you can iterate over a string as chars or bytes with iterators
    let foo = String::from("フー・バー");

    for byte in foo.bytes() {
        println!("{byte}");
    }

    for char in foo.chars() {
        println!("{char}");
    }

    println!("{foo}");
}