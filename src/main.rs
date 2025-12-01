use std::fs;

use crate::day01::solve;

mod day01;

fn main() {
    let contents = fs::read_to_string("inputs/day01.txt").expect("Can't do nothing without input");
    let solution = solve(&contents);
    println!("Solution: {}", solution);
}
