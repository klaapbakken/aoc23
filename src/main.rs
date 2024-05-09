pub mod read;
pub mod day1;
pub mod day2;
pub mod day3;

use std::env;

#[allow(unused_imports)]
use day1::solve_day_1;
use day2::solve_day_2;
use day3::solve_day_3;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: &String = &args[1];
    let path = &args[2];

    match day.as_str() {
        "1" => solve_day_1(path),
        "2" => solve_day_2(path),
        "3" => solve_day_3(path),
        _ => ()
    }

}