//Like the new module style? Hopefully it's not as intimidating as the 200 line files!
mod reading;

pub fn explain() {
    println!("How does one get values out of a vector? Let's find out!");

    // quick_review();

    reading::explain();
}

fn quick_review() {
    println!("But before that, let's have a quick reivew on the module system!");
    /*
    
    Would this compile if placed into src/vectors.rs?
    {
        mod using_values
        
        using_values::quick_review();
    }
    
    Go test it out!
    */
}