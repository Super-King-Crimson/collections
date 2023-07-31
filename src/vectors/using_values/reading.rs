pub fn explain() {
    println!("There are two ways to reference a value in a vector: the get method and indexing.");

    let hard_kanji = vec!['痙', '攣'];

    //Make sure to use references - we don't want to take ownership of the items in the vector
    //(I mean, we're using stuff on the stack so it's automatically cloned but still)
    let spa: &char = &hard_kanji[0];
    let sm: Option<&char> = hard_kanji.get(1);
    //As you can see, the get method returns an option to a reference (so it's safe w/all types)
    
    //The reason why there are two methods is so you can choose what happens when you go past a vector's bounds

    //Stops program with error (FROM THE FUTURE: It's actually called 'panicking')
    // let panic = hard_kanji[10];

    //Handles error
    let handle: &char = match hard_kanji.get(10) {
        Some(val) => &val,
        None => &'無',  
    };
}