use std::fs;
use std::collections::HashMap;

pub fn main() {
    let data = fs::read_to_string("./data/day2.txt").unwrap();

    println!("{}", part_one(&data));
    println!("{}", part_two(&data));
}


fn part_one(data: &str) -> i32 {
    let lines = data.lines();

    let colour_max_values = HashMap::from([
        ("red", 12),
        ("green", 13), 
        ("blue", 14)
    ]);

    let mut total = 0;

    for line in lines {
        let line_parts: Vec<&str> = line.split(":").take(2).collect();
        let game_id = line_parts[0];
        let cubes_drawn = line_parts[1];

        let mut possible = true;

        let cubes_num_colour: Vec<&str> = cubes_drawn.split([',', ';']).collect();
        for cube_num_colour in cubes_num_colour {

            let num_colour: Vec<&str> = cube_num_colour.trim().split_ascii_whitespace().collect();
            let num: i32 = num_colour[0].parse().unwrap();
            let colour: &str = num_colour[1];

            let max_value = colour_max_values.get(colour).unwrap().to_owned();
            if num > max_value {
                possible = false;
            }
        }

        if possible {
            total += game_id.strip_prefix("Game ").unwrap().parse::<i32>().unwrap();
        }
    };

    total
}

fn part_two(data: &str) -> i32 {
    let lines = data.lines();

    let mut total = 0;

    for line in lines {
        let mut highest_colour_value = HashMap::from([
            ("red", 0),
            ("green", 0), 
            ("blue", 0)
        ]);

        let line_parts: Vec<&str> = line.split(":").take(2).collect();
        let cubes_drawn = line_parts[1];

        let cubes_num_colour: Vec<&str> = cubes_drawn.split([',', ';']).collect();
        for cube_num_colour in cubes_num_colour {
            let num_colour: Vec<&str> = cube_num_colour.trim().split_ascii_whitespace().collect();

            let number = num_colour[0].parse::<i32>().unwrap();
            let colour = num_colour[1];

            if highest_colour_value.get(colour).unwrap().to_owned() < number {
                highest_colour_value.insert(colour, number);
            }
        }

        total += highest_colour_value.values().product::<i32>();
    }

    total
}