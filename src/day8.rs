use std::{fs, collections::HashMap};
use regex::{Regex, Captures};

pub fn main() {
    let data = fs::read_to_string("./data/day7.txt").unwrap();

    println!("{}", part_one(&data));
    // println!("{}", part_two(&data));
}

fn part_one<'a>(data: &'a str) -> usize {
    let re = Regex::new(r"(?P<start>\w{3}) = \((?P<left>\w{3}), (?P<right>\w{3})\)").unwrap();

    let data_lines = data.lines();

    let mut vals: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in data_lines {
        let values: Captures<'a> = re.captures(line).unwrap();
        vals.insert(&values["start"], (&values["left"], &values["right"]));
    };

    0
}

// fn part_two(data: &str) -> usize {

// }