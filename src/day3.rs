use crate::read;
use std::collections::HashSet;

pub fn solve_day_3(path: &String) -> () {
    println!("Day 3 - Part 1: {}", solve_day_3_part_1(&path));
}

fn find_symbols(rows: &Vec<Vec<char>>) -> HashSet<char>{
    let mut symbols = HashSet::new();
    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            let element = rows[i][j];
            if !element.is_alphanumeric() && element != '.' {
                symbols.insert(element);
            }
        }
    }
    return symbols;
}

pub fn solve_day_3_part_1(path: &String) -> u32 {
    let lines = read::read_lines(path);
    let mut rows = Vec::<Vec<char>>::new();
    for line in lines {
        rows.push(line.chars().collect());
    }
    let symbols = find_symbols(&rows);
    // for symbol in symbols {
    //     println!("{}", symbol);
    // }

    let mut part_number_elements: Vec<char> = Vec::<char>::new();
    let mut running_total = 0;
    for i in 0..rows.len() {
        // println!("i={:?}", i);
        for j in 0..rows[i].len() {
            // println!("j={:?}", j);
            let element = rows[i][j];
            if element >= '0' && element <= '9' {
                part_number_elements.push(element);
            } else {
                if part_number_elements.len() > 0 {
                    let part_number: u32 = part_number_elements.iter().collect::<String>().parse::<u32>().expect("Couldn't parse part number.");
                    let j_start = j - part_number_elements.len();
                    let j_end = j;
                    let neighbour_positions = get_neighbour_positions(&j_start, &j_end, &i);
                    if check_neighbors(&rows, &neighbour_positions, &symbols) {
                        running_total = running_total + part_number;
                    }
                }
                part_number_elements.clear();
            }
        }
        if part_number_elements.len() > 0 {
            let part_number: u32 = part_number_elements.iter().collect::<String>().parse::<u32>().expect("Couldn't parse part number.");
            let j_start = rows[i].len() - part_number_elements.len();
            let j_end = rows[i].len() - 1;
            let neighbour_positions = get_neighbour_positions(&j_start, &j_end, &i);
            if check_neighbors(&rows, &neighbour_positions, &symbols) {
                running_total = running_total + part_number;
            }
        }
        part_number_elements.clear();
    }
    return running_total;
}

fn get_neighbour_positions(j_start: &usize, j_end: &usize, i: &usize) -> Vec<(isize, isize)> {
    let mut positions = HashSet::<(isize, isize)>::new();
    for j in *j_start..*j_end {
        let i_signed = i.clone() as isize;
        let j_signed = j.clone() as isize;
        let element_neighbours = [
            (i_signed - 1, j_signed - 1),
            (i_signed - 1, j_signed),
            (i_signed - 1, j_signed + 1),
            (i_signed, j_signed - 1),
            (i_signed, j_signed),
            (i_signed, j_signed + 1),
            (i_signed + 1, j_signed - 1),
            (i_signed + 1, j_signed),
            (i_signed + 1, j_signed + 1)
        ];
        for neighbour in element_neighbours {
            positions.insert(neighbour);
        }
    }
    return positions.into_iter().collect()
}

fn check_neighbors(rows: &Vec<Vec<char>>, positions: &Vec<(isize, isize)>, symbols: &HashSet<char>) -> bool {
    let zero = 0 as isize;
    let n_rows = rows.len() as isize;
    for (u, v) in positions.iter() {
        if u >= &zero && v >= &zero {
            let u_unsigned = u.abs() as usize;
            let v_unsigned = v.abs() as usize;
            if u < &n_rows {
                let n_cols = rows[u_unsigned].len() as isize;
                if v < &n_cols {
                    match symbols.get(&rows[u_unsigned][v_unsigned]) {
                        Some(_) => return true,
                        None => continue
                    };
                }
            }
        }
    }
    return false;
}