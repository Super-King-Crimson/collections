use std::slice::Iter;

pub fn explain() {
    //We'll talk about iterators more in-depth later, but know that iterators have pointers to vec data
    //Here's a breakdown of what a for loop is actually doing (for loop de-sugared)

    let mut v: Vec<i32>         = vec![1, 2];
    let mut iter: Iter<'_, i32> = v.iter();             //iter initialized, looking at 1
    let n1: &i32                = iter.next().unwrap(); //1, iter looks at 2
    let n2: &i32                = iter.next().unwrap(); //2, iter is done moving frees own memory
    let end: Option<&i32>       = iter.next();          //iter has nothing left to return now, so this is None

    //In other words, an iterator is actually a pointer!
    //The next method moves the iterator to the next position in the vector, 
        //and returns an Option reference to the element it was previously on (None at the end of the list)

    //Why does this matter? Let's say we wanted to duplicate a vector in place, so [1, 2] became [1, 2, 1, 2]
    let mut double_it = vec![1, 2];
    dup_in_place(&mut double_it);
    dup_in_place_fr(&mut double_it);

    println!("Here's the doubled vec! {double_it:?}");
}

//For some reason, iterating over a vector removes its W perms! Why?
fn dup_in_place(vec: &mut Vec<i32>) {
    for num in vec.iter() {
        //vec.push(*num);
    }
}

//Remember how resizing a vec can make it move? 
//Since an iterator's a pointer, this push could move the vector and make the iterator point to invalid data

fn dup_in_place_fr(vec: &mut Vec<i32>) {
    //Here's an actual implementation: We use a numerical iterator (a range) instead of a pointer
    for i in 0..vec.len() {
        vec.push(vec[i]);
    }
}