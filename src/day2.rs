use crate::read;
use std::collections::HashMap;

fn observe_cube(line: &String) -> (u32, HashMap<&str, Vec<u32>>) {
    let mut cube_observations: HashMap<&str, Vec<u32>> = HashMap::new();

    let game_vec: Vec<&str> = line.split(":").collect();
    let game_number: u32 = game_vec[0].trim_start_matches("Game ").parse::<u32>().expect("Could not parse game number.");
    let sets_vec = game_vec[1].split(";");
    for set in sets_vec {
        let cubes_vec: Vec<&str> = set.split(",").collect();
        for mut cube in cubes_vec {
            cube = cube.trim().trim_end();
            let amount_str = cube.split(" ").nth(0).expect("Amount not found.");
            let amount = amount_str.parse::<u32>().expect("Could not parse amount.");
            let color = cube.split(" ").nth(1).expect("Color not found.");
            let mut current_color_amount = match cube_observations.get(&color) {
                Some(amount_vec) => amount_vec.clone(),
                None => Vec::<u32>::new()
            };
            current_color_amount.push(amount);
            cube_observations.insert(color, current_color_amount);
        }
    }
    return (game_number, cube_observations);
}


fn game_number_if_valid(line: &String) -> u32 {
    let game_number: u32;
    let cube_observations: HashMap<&str, Vec<u32>>;
    
    (game_number, cube_observations) = observe_cube(line);
    let observed_reds = get_max_obs("red", &cube_observations);
    let observed_greens = get_max_obs("green", &cube_observations);
    let observed_blues = get_max_obs("blue", &cube_observations);
    let valid = *observed_reds <= 12 && *observed_greens <= 13 && *observed_blues <= 14;
    if valid {
        return game_number
    } else {
        return 0
    }

}

fn power_of_minimum_set(line: &String) -> u32 {
    let cube_observations: HashMap<&str, Vec<u32>>;
    
    (_, cube_observations) = observe_cube(line);
    let observed_reds = get_max_obs("red", &cube_observations);
    let observed_greens = get_max_obs("green", &cube_observations);
    let observed_blues = get_max_obs("blue", &cube_observations);
    observed_reds * observed_greens * observed_blues

}

fn get_max_obs<'a>(color: &'a str, obs: &'a HashMap<&str, Vec<u32>>) -> &'a u32 {
    let observed: &'a u32 = match obs.get(color) {
        Some(amount) => amount.iter().max().unwrap_or(&0),
        None => &0
    };
    return observed;
}

pub fn solve_day_2(path: &String) -> () {
    let lines = read::read_lines(path);
    let sum_of_valid_games = lines.iter().map(|line| game_number_if_valid(line)).sum::<u32>();
    let sum_of_power_of_minimum_set = lines.iter().map(|line| power_of_minimum_set(line)).sum::<u32>();
    println!("Day 2 - Part 1: {sum_of_valid_games}");
    println!("Day 2 - Part 2: {sum_of_power_of_minimum_set}");
}
