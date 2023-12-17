
use std::{fs, char, collections::HashMap};

pub fn main() {
    let data = fs::read_to_string("./data/day3.txt").unwrap();

    println!("{}", part_one(&data));
    println!("{}", part_two(&data));
}


fn part_one(data: &str) -> usize {

    let data_lines = data.lines();

    let data_vec: Vec<Vec<char>> = data_lines.map(|x| x.chars().collect()).collect();

    let mut total = 0;

    let mut current_is_part = false;
    let mut current: Vec<&char> = vec![];

    for (row_index, line) in data_vec.iter().enumerate() {
        for (column_index, character) in line.iter().enumerate() {
            if character.is_numeric() {
                current.push(character);
                if check_character(&data_vec, row_index as i32, column_index as i32){
                    current_is_part = true;
                }
            } else {
                if !current.is_empty() {
                    if current_is_part {
                        let part_string: String = current.into_iter().collect();
                        total += part_string.parse::<usize>().unwrap();
                    }
                    current = vec![];
                    current_is_part = false;
                }
            }
        }
    };


    total
}

fn check_character(data_vec: &Vec<Vec<char>>, row: i32, column: i32) -> bool {
    for row_index in row-1..row+2 {
        for column_index in column-1..column+2 {
            if row_index >= 0 && column_index >= 0 {
                if let Some(row_data) = data_vec.get(row_index as usize) {
                    if let Some(column_data) = row_data.get(column_index as usize) {
                        if column_data.is_ascii_punctuation() && column_data != &'.' {
                            return true
                        }
                    }
                }
            }
        }
    };

    return false;
}










fn part_two(data: &str) -> usize {

    let data_lines = data.lines();

    let data_vec: Vec<Vec<char>> = data_lines.map(|x| x.chars().collect()).collect();

    let mut total = 0;

    let mut current_is_part: Vec<(usize, usize)> = Vec::new();
    let mut current: Vec<&char> = vec![];

    let mut part_counts: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for (row_index, line) in data_vec.iter().enumerate() {
        for (column_index, character) in line.iter().enumerate() {
            if character.is_numeric() {
                current.push(character);
                let gear_pos = check_gear(&data_vec, row_index as i32, column_index as i32);
                if let Some(gear_pos) = gear_pos {
                    if !current_is_part.contains(&gear_pos){
                        current_is_part.push(gear_pos)
                    }
                }
            } else {
                if !current.is_empty() {
                    if current_is_part.len() > 0 {
                        let part_string: String = current.into_iter().collect();
                        let numb = part_string.parse::<usize>().unwrap();
                        for gear_pos in current_is_part {
                            let mut value = vec![numb];
                            if let Some(jjj) = part_counts.get(&gear_pos) {
                                for aaa in jjj {
                                    value.push(aaa.to_owned());
                                }
                            }
                            part_counts.insert(gear_pos,value);
                        }
                    }
                    current = vec![];
                    current_is_part = vec![];
                }
            }
        }
    };


    for val in part_counts.values() {
        if val.len() > 1 {
            let mut tempss = 1;
            for nummms in val {
                tempss = tempss * nummms;
            }
            println!("{}", tempss);
            total += tempss;
        }
    };

    total
}

fn check_gear(data_vec: &Vec<Vec<char>>, row: i32, column: i32) -> Option<(usize, usize)> {
    for row_index in row-1..row+2 {
        for column_index in column-1..column+2 {
            if row_index >= 0 && column_index >= 0 {
                if let Some(row_data) = data_vec.get(row_index as usize) {
                    if let Some(column_data) = row_data.get(column_index as usize) {
                        if column_data == &'*' {
                            return Some((column_index as usize, row_index as usize))
                        }
                    }
                }
            }
        }
    };

    None
}