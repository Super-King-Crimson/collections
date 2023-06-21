pub mod safety;

pub fn explain() {
    //Use a for loop. What did you expect?
    let mut rotcev = vec![5, 4, 3, 2, 1];
    
    let mut prev_sum = 0;
    let mut post_sum = 0;

    //Be a good Rustacean and don't randomly take ownership of things!
    for num in &mut rotcev {
        //Have to deref here: no implicit deref for addition of mutable references to numbers
        prev_sum += *num;
        *num += 10;
        post_sum += *num;
    }

    println!("The sum of the values in the vec was {prev_sum}, but now it's {post_sum}.");

    safety::explain();
}