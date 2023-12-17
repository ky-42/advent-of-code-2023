use std::{fs, collections::{HashSet, HashMap}};

pub fn main() {
    let data = fs::read_to_string("./data/day4.txt").unwrap();

    println!("{}", part_one(&data));
    println!("{}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let data_line = data.lines();

    let mut total = 0;

    for line in data_line {
        let match_number = check_card_matches(line);

        if let Some(match_number) = match_number.checked_sub(1) {
           total += usize::pow(2, match_number);
        }
    };
    total
}

fn part_two(data: &str) -> usize {
    let data_line = data.lines();

    let mut total: HashMap<usize, usize> = HashMap::from([(1,1)]);

    for (index, line) in data_line.enumerate() {

        total.entry(index+1).or_insert(1);

        let card_score: usize = check_card_matches(line) as usize;

        for num in index+2..card_score+index+2 {
            total.insert(num, total.get(&num).unwrap_or(&1)+(total.get(&(index+1)).unwrap_or(&1)));
        }
    };
    total.values().sum()
}

fn check_card_matches(line: &str) -> u32 {
    let mut card_score = 0;

    let mut winning_nums = HashSet::new();
    let mut adding_winners = true;

    let line_words: Vec<&str> = line.split_ascii_whitespace().collect();
    for word in line_words[2..].iter() {
        if word == &"|" {
            adding_winners = false;
        } else if adding_winners {
            winning_nums.insert(word);
        } else {
            if winning_nums.contains(word) {
                card_score += 1;
            }
            
        }
    };

    card_score
}