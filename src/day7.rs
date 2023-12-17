

use std::{fs, collections::{HashSet, HashMap}, ops::{Index, IndexMut}, cmp::Ordering};


pub fn main() {
    let data = fs::read_to_string("./data/day7.txt").unwrap();

    println!("{}", part_one(&data));
    println!("{}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let data_lines = data.lines();

    let matches: [(usize, usize); 7] = [(0, 0), (2,0), (2, 2), (3, 0), (3, 2), (4, 0), (5,0)];
    let mut card_groups: [Vec<&str>; 7] = Default::default();
    let letter_values =  HashMap::from([
        ('A', 14),
        ('K', 13), 
        ('Q', 12),
        ('J', 11), 
        ('T', 10), 
    ]);


    for line in data_lines {
        let aaa: Vec<&str> = line.split_ascii_whitespace().collect();

        let mut char_count: HashMap<char, usize> = HashMap::new();

        for character in aaa[0].chars() {
            char_count.entry(character).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut pairs_filter = char_count.values().filter(|f| **f > 1);

        let (first, second) = (*pairs_filter.next().unwrap_or(&0), *pairs_filter.next().unwrap_or(&0));
        let mut pairs = [first, second];
        pairs.sort();
        let pairs = (pairs[1], pairs[0]);

        card_groups.index_mut(matches.iter().position(|&x| x == pairs).unwrap()).push(&line);
    }

    for card_group in &mut card_groups {
        card_group.sort_by(|a, b| {
            let a = a.split_ascii_whitespace().next().unwrap();
            let b = b.split_ascii_whitespace().next().unwrap();
            for abc in a.chars().zip(b.chars()) {
                let first = *letter_values.get(&abc.0).unwrap_or(&abc.0.to_string().parse().unwrap_or(0));
                let second = *letter_values.get(&abc.1).unwrap_or(&abc.1.to_string().parse().unwrap_or(0));
                if first > second {
                    return Ordering::Greater;
                } else if first < second {
                    return Ordering::Less;
                } else {
                    continue;
                }
            };
            Ordering::Equal
        });
    }


    let mut total = 0;
    let mut index = 1;
    for card_group in card_groups.iter() {
        for card in card_group {
            let no: Vec<&str> = card.split_ascii_whitespace().collect();
                total += no[1].parse::<usize>().unwrap() * index;
                index += 1;
        }
    };

    total
}

fn part_two(data: &str) -> usize {
    let data_lines = data.lines();

    let matches: [(usize, usize); 7] = [(0, 0), (2,0), (2, 2), (3, 0), (3, 2), (4, 0), (5,0)];
    let mut card_groups: [Vec<&str>; 7] = Default::default();
    let letter_values =  HashMap::from([
        ('A', 14),
        ('K', 13), 
        ('Q', 12),
        ('T', 10), 
        ('J', -1), 
    ]);


    for line in data_lines {
        let aaa: Vec<&str> = line.split_ascii_whitespace().collect();

        let mut char_count: HashMap<char, usize> = HashMap::new();

        let mut j_count = 0;
        for character in aaa[0].chars() {
            if character == 'J' {
                j_count += 1;
                continue;
            }
            char_count.entry(character).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut pairs_filter = char_count.values().filter(|f| **f > 1);

        let (first, second) = (*pairs_filter.next().unwrap_or(&0), *pairs_filter.next().unwrap_or(&0));
        let mut pairs = [first, second];
        pairs.sort();
        let mut pairs = (pairs[1]+j_count, pairs[0]);

        if pairs.0 == j_count && j_count != 5 && j_count != 0 {
            pairs.0 += 1;
        }

        println!("{:#?}", pairs);

        card_groups.index_mut(matches.iter().position(|&x| x == pairs).unwrap()).push(&line);
    }

    
    for card_group in &mut card_groups {
        card_group.sort_by(|a, b| {
            let a = a.split_ascii_whitespace().next().unwrap();
            let b = b.split_ascii_whitespace().next().unwrap();
            for abc in a.chars().zip(b.chars()) {
                let first = *letter_values.get(&abc.0).unwrap_or(&abc.0.to_string().parse().unwrap_or(0));
                let second = *letter_values.get(&abc.1).unwrap_or(&abc.1.to_string().parse().unwrap_or(0));
                if first > second {
                    return Ordering::Greater;
                } else if first < second {
                    return Ordering::Less;
                } else {
                    continue;
                }
            };
            Ordering::Equal
        });
    }
    
    println!("{:#?}", card_groups);
    
    let mut total = 0;
    let mut index = 1;
    for card_group in card_groups.iter() {
        for card in card_group {
            let no: Vec<&str> = card.split_ascii_whitespace().collect();
                // println!("{} {}", no[1], index);
                total += no[1].parse::<usize>().unwrap() * index;
                index += 1;
        }
    };

    total
}