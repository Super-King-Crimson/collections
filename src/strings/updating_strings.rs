pub fn explain() {
    //There are several ways to change the contents of a string: concatenation (+), format! and push (like a vec)
    
    let mut foo = String::from("foo"); //foo

    //pushes an entire &str onto a string in place
    foo.push_str("bar"); //foobar

    //pushes a singular char onto a string in place
    foo.push('b'); //foobarb
    //Both methods don't take ownership of the string because then we wouldn't be able to use it afterwards

    //Joins two strings and returns the new one (takes ownership of foo)
    let baz = foo + &String::from("az"); //foobarbaz
    //This is actually the add method, which accepts self and a str slice to add, and returns a new String
    
    //If you're planning on combining a bunch of different strings, use the format! macro
    let strs: [String; 4] = [String::from("foo"), String::from("bar"), String::from("baz"), String::from("qux")];

    //Doesn't take ownership of anything, returns a new string
    let qux = format!("{} + {} + {} + {} = {}",  //foo + bar + baz + qux = foobarbazqux
        strs[0], strs[1], strs[2], strs[3], format!("{}{}{}{}", strs[0], strs[1], strs[2], strs[3]));
}