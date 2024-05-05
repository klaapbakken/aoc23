use crate::read;
use std::cmp::min;


#[derive(Debug)]
struct Digit {
    string: &'static str,
    numeric: char
}

impl Digit {
    fn string_length(&self) -> usize {
        self.string.chars().count()
    }
}

pub fn solve_day_1(path: &String) -> (){
    let lines = read::read_lines(path);

    println!("Day 1 - Part 1: {}", solve_day1_part1(lines.clone()));
    println!("Day 1 - Part 2: {}", solve_day1_part2(lines.clone()));
}

fn is_digit(char: char) -> bool {
    char >= '0' && char <= '9'
}

pub fn solve_day1_part1(lines: Vec<String>) -> u32 {
    let mut first_digit: Option<char>;    
    let mut last_digit: Option<char>;
    let mut combined;
    let mut sum = 0;

    for line in lines {
        let mut first = true;
        first_digit = None;
        last_digit = None;
        for char in line.chars() {
            if is_digit(char) {
                if first {
                    first_digit = Some(char);
                    first = false;
                }
                last_digit = Some(char);
            }
        }
        combined = first_digit.unwrap_or('0').to_string() +
            &last_digit.unwrap_or('0').to_string();
        sum = sum + combined.parse::<u32>().expect("Could not parse as integer.")
    }
    return sum;
}

fn find_digits(line: &String) -> String {
    let digits: Vec<Digit> = vec![
        Digit { string: "one", numeric: '1'},
        Digit { string: "two",  numeric: '2'},
        Digit { string: "three",  numeric: '3'},
        Digit { string: "four",  numeric: '4'},
        Digit { string: "five",  numeric: '5'},
        Digit { string: "six",  numeric: '6'},
        Digit { string: "seven",  numeric: '7'},
        Digit { string: "eight",  numeric: '8'},
        Digit { string: "nine", numeric: '9'}
    ];
    
    let char_vec: Vec<char> = line.chars().collect();
    let n_chars = char_vec.len();
    let mut new_line = String::new();
    for i in 0..n_chars {
        for digit in &digits {
            if char_vec[i] == digit.numeric {
                new_line = new_line + &digit.numeric.to_string();
            } else { 
                let last_index = min(i + digit.string_length(), n_chars);
                let potential_match = String::from_iter(char_vec[i..last_index].iter());
                if potential_match == digit.string {
                    new_line = new_line + &digit.numeric.to_string();
                }
            }
        }
    }
    return new_line;
}


pub fn solve_day1_part2(lines: Vec<String>) -> u32 {

    let mut new_lines: Vec<String> = vec![];
    for line in lines {
        let mut new_line = line;
        new_line = find_digits(&new_line);
        new_lines.push(new_line);
    }

    solve_day1_part1(new_lines)

}