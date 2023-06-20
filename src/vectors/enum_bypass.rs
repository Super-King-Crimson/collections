//Ok, remember when I said vectors can only store one type? Remember enums? Yeah.
enum Number {
    Int(i64),
    Double(f64),
}

enum Types {
    Bool(bool),
    Char(char, char),
    Num(Number),
}

use Types::{Bool, Char, Num};

pub fn explain() {
    let mut cheater: Vec<Types> = vec![
        Bool(true),
        Char('f', 'r'),
        Num(Number::Double(1.3)),
    ];

    for thing in &mut cheater {
        match thing {
            Bool(_) => println!("It's a bool!"),
            Char(c1, c2) => println!("Here's your message: {c1}{c2}"),

            Num(n) => match n {
                Number::Int(_) => println!("This is an integer!"),
                Number::Double(d) => *d += 100f64,
            },
        }
    }
    //Hey, it's safe!
    //As long as you know everything that's gonna be in the vector, you can cheat the type system any time.

    //If you don't know the type of everything, try a trait (we'll talk about it later baybee!!!)
}