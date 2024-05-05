pub mod read;
pub mod day1;

use std::env;

#[allow(unused_imports)]
use day1::{solve_day_1, solve_day1_part1, solve_day1_part2};

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: &String = &args[1];
    let path = &args[2];

    match day.as_str() {
        "1" => solve_day_1(path),
        _ => ()
    }

}