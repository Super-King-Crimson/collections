pub mod using_values;
pub mod iteration;
pub mod enum_bypass;

pub fn explain() {
    println!("A vector's technical name is Vec<T>.");

    //Vectors allow you to store a bunch of values of the same type next to each other in memory
    //It's good for storing lists that can change, like a list of items in a shopping cart

    //This is how you create a new, empty vector:
    let vector: Vec<i32> = Vec::new();
    //We need to add a type annotation if we do it like this - it's impossible for the compiler to infer the type
    //That <T> is replaced with a type, once declared the vector can only hold items of that type

    //You're better off using the vec! macro to create a vector:
    let mut v = vec!('a', 'b', 'c');
    //The compiler can infer this one!

    //To add values to a vector, use push (obvs requires vec to be mutable)
    v.push('d');

    //When a vector is dropped, its contents are dropped (a vector is the owner of all values inside of it)
    {
        let tor = vec![0.1, 0.2, 0.3];
    }   //And the doubles were never seen again...

    using_values::explain();
    iteration::explain();
    enum_bypass::explain();

    //Make sure to review the docs for vectors! There's a lot more that can be done, like pop
}