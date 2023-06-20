#![allow(unused, dead_code)]
use homepage as topic;
mod vectors;

fn main() {
    // topic::introduce();
    vectors::explain();
}

mod homepage {
    pub fn introduce() {
        println!(
            "CHAPTER 8: COMMON COLLECTIONS

            Collections are unique in Rust: not only can they contain multiple values like a tuple or array,
            but they're also stored on the heap, so the compiler doesn't need to know how much data they hold.

            We'll be going over the three most common Rust collections:
                - Vectors
                - Strings
                - Hash maps
            There are more, which can be found on the Rust documentation.
            "
        );
    }
}