use std::env;

mod aoc01_1;
mod aoc01_2;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    println!("aoc01_1");
    aoc01_1::main();
    println!("aoc01_2");
    aoc01_2::main();
}