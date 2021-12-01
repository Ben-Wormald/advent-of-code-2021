#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_01; }
use solutions::day_01::solve;
const INPUT: &str = "./input/01";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
