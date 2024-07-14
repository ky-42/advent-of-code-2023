use std::{fs, collections::HashMap};
use regex::{Regex, Captures};

pub fn main() {
    let data = fs::read_to_string("./data/day8.txt").unwrap();

    // println!("{}", part_one(&data));
    println!("{}", part_two(&data));
}

fn part_one<'a>(data: &'a str) -> usize {
    let re = Regex::new(r"(?P<start>\w{3}) = \((?P<left>\w{3}), (?P<right>\w{3})\)").unwrap();

    let mut data_lines = data.lines();
    let instructions: Vec<char> = data_lines.next().unwrap().chars().collect();
    data_lines.next().unwrap();

    let mut vals: HashMap<String, (String, String)> = HashMap::new();

    for line in data_lines {
        let values: Captures<'a> = re.captures(line).unwrap();
        vals.insert(values["start"].to_string(), (values["left"].to_string(), values["right"].to_string()));
    };

    let mut steps = 1;
    let mut index = 0;
    let mut options = vals.get("AAA").unwrap();
    
    loop {
        let temp = index % instructions.len();
        let instruction = instructions[temp];
        let next_element: &str;
        if instruction == 'L' {
            next_element = &options.0;
        } else {
            next_element = &options.1;
        }

        if next_element == "ZZZ" {
            return steps
        } else {
            // println!("{:?} {} {}", options, next_element, instruction);
            index += 1;
            steps += 1;
            options = vals.get(next_element).unwrap();
        }
    };
}

fn part_two<'a>(data: &'a str) -> usize {
    let re = Regex::new(r"(?P<start>\w{3}) = \((?P<left>\w{3}), (?P<right>\w{3})\)").unwrap();

    let mut data_lines = data.lines();
    let instructions: Vec<char> = data_lines.next().unwrap().chars().collect();
    data_lines.next().unwrap();

    let mut vals: HashMap<String, (String, String)> = HashMap::new();
    let mut starts: Vec<String> = vec![];

    for line in data_lines {
        let values: Captures<'a> = re.captures(line).unwrap();
        let start = values["start"].to_string();
        if start.ends_with("A") {
            starts.push(start.clone());
        }
        vals.insert(values["start"].to_string(), (values["left"].to_string(), values["right"].to_string()));
    };

    let mut steps = 1;
    let mut index = 0;
    let mut options: Vec<&(String, String)> = starts.iter().map(|x| vals.get(x).unwrap()).collect();

    
    loop {
        let mut nexts: Vec<&String> = vec![];
        let temp = index % instructions.len();
        let instruction = instructions[temp];
        // println!("{:?}", options);
        for option in options {
            let next_element: &String;
            if instruction == 'L' {
                next_element = &option.0;
            } else {
                next_element = &option.1;
            }

            nexts.push(next_element);

        }
        // println!("{:?} {} \n\n", nexts, instruction);
        
        if nexts.iter().all(|x| x.ends_with("Z")) {
            return steps
        }
        options = nexts.iter().map(|x| vals.get(*x).unwrap()).collect();
        index += 1;
        steps += 1;

        // if steps > 1 {
        //     // println!("{}", steps);
        //     return 0;
        // }
    };
}