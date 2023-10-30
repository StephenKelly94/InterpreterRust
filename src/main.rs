pub mod lexer;
use std::{env, fs, print};

fn main() {
    let file = env::args().nth(1).expect("No file provided");

    let contents = fs::read_to_string(file).expect("No contents");

    print!("{:?}", contents);
}
