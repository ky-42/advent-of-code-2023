use std::fs;

pub fn main() {
    let data = fs::read_to_string("../data/day1.txt").unwrap();

    println!("{}", part_one(&data));
    println!("{}", part_two(&data));
}

fn part_one(data: &str) -> i32 {
    let lines = data.lines();

    let mut total = 0;

    for line in lines {
        let mut first_num = char::from(0);
        let mut second_num = char::from(0);

        for letter in line.chars() {
            if letter.is_ascii_digit() {
                first_num = letter;
                break
            };
        };

        for letter in line.chars().rev() {
            if letter.is_ascii_digit() {
                second_num = letter;
                break
            };
        };

        total += format!("{}{}", first_num, second_num).parse::<i32>().unwrap();
    };

    total
}

fn part_two(data: &str) -> i32 {
    let lines = data.lines();

    let mut total = 0;

    for line in lines {
        let line_length = line.len();
        let mut line_numbers: Vec<char> = vec![];

        for (index, letter) in line.chars().enumerate() {
            match letter {
                'o' => {
                    if  index + 2 < line_length && &line[index..index+3] == "one" {
                        line_numbers.push('1');
                    }
                },
                't' => {
                    if index + 2 < line_length && &line[index..index+3] == "two" {
                        line_numbers.push('2')
                    }
                    if index + 4 < line_length && &line[index..index+5] == "three" {
                        line_numbers.push('3')
                    }
                },
                'f' => {
                    if index + 3 < line_length && &line[index..index+4] == "four" {
                        line_numbers.push('4')
                    }
                    if index + 3 < line_length && &line[index..index+4] == "five" {
                        line_numbers.push('5')
                    }
                },
                's' => {
                    if index + 2 < line_length && &line[index..index+3] == "six" {
                        line_numbers.push('6')
                    }
                    if index + 4 < line_length && &line[index..index+5] == "seven" {
                        line_numbers.push('7')
                    }
                },
                'e' => {
                    if index + 4 < line_length && &line[index..index+5] == "eight" {
                        line_numbers.push('8')
                    }
                },
                'n' => {
                    if index + 3 < line_length && &line[index..index+4] == "nine" {
                        line_numbers.push('9')
                    }
                },
                _ => {
                    if letter.is_ascii_digit() {
                        line_numbers.push(letter);
                    };
                }
            };
        }

        total += format!("{}{}", line_numbers[0], line_numbers[line_numbers.len()-1]).parse::<i32>().unwrap();
        
    };

    total
}
