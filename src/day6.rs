
use std::fs;

pub fn main() {
    let data = fs::read_to_string("./data/day6.txt").unwrap();

    println!("{}", part_one(&data));
    println!("{}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let mut data_lines = data.lines();
    let times: Vec<isize> = data_lines.nth(0).unwrap().strip_prefix("Time:").unwrap().split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    let distances: Vec<isize> = data_lines.nth(0).unwrap().strip_prefix("Distance:").unwrap().split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut vals: Vec<usize> = vec![];

    for (time, dist) in times.into_iter().zip(distances) {
        let part1 = (time * -1) as f64;
        let part2 = ((time.pow(2)-(4*(dist + 1))) as f64).sqrt();

        let left = ((part1 + part2)/-2.0).ceil() as usize;
        let right = ((part1 - part2)/-2.0).floor() as usize;

        vals.push(right-left+1); 
    };

    println!("{:#?}", vals);
    vals.iter().fold(1, |acc, &x| acc * x)
}

fn part_two(data: &str) -> usize {
    let mut data_lines = data.lines();
    let time: isize = data_lines.nth(0).unwrap().strip_prefix("Time:").unwrap().split_ascii_whitespace().fold(String::new(), |a, b| a + b).parse().unwrap();
    let dist: isize = data_lines.nth(0).unwrap().strip_prefix("Distance:").unwrap().split_ascii_whitespace().fold(String::new(), |a, b| a + b).parse().unwrap();


    let part1 = (time * -1) as f64;
    let part2 = ((time.pow(2)-(4*(dist + 1))) as f64).sqrt();

    let left = ((part1 + part2)/-2.0).ceil() as usize;
    let right = ((part1 - part2)/-2.0).floor() as usize;

    right-left+1 
}