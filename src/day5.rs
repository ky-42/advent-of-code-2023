use std::{fs, vec};

pub fn main() {
    let data = fs::read_to_string("./data/day5.txt").unwrap();

    println!("{}", part_one(&data));
    println!("{}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let mut data_lines: Vec<&str> = data.lines().collect(); 

    let mut current: Vec<usize> = data_lines[0].strip_prefix("seeds: ").expect("1").split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut next: Vec<usize> = vec![]; 

    let mut abc: Vec<Vec<usize>> = vec![];

    let (_, data_lines) = data_lines.split_at_mut(2);

    for line in data_lines {
        if line.contains("map") { continue; }
        if line.is_empty() {
            for cun in current.iter() {
                let mut added = false;
                for val in abc.iter() {
                    if *cun >= val[1] && *cun < val[1]+val[2] {
                        let temp = cun-val[1];
                        next.push(val[0]+temp);
                        added = true;
                        break;
                    }
                }
                
                if !added {
                    next.push(*cun);
                }
            }
            
            current = next;
            next = vec![];
            abc = vec![];
            continue;
        }

        let numbers:Vec<usize> = line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
        abc.push(numbers);

    };
    
    for cun in current.iter() {
        let mut added = false;
        for val in abc.iter() {
            if *cun >= val[1] && *cun < val[1]+val[2] {
                let temp = cun-val[1];
                next.push(val[0]+temp);
                added = true;
                break;
            }
        }
        
        if !added {
            next.push(*cun);
        }
    }
    
    current = next;
    *current.iter().min().unwrap()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DataRange {
    start: usize,
    end: usize,
    change: Option<usize>
}

impl DataRange {
    fn overlaps(&self, other: &DataRange) -> Option<DataRange> {
        if !(self.end < other.start || other.end < self.start) {
            Some(DataRange {
                start: self.start.max(other.start),
                end: self.end.min(other.end),
                change: None
            })
        } else {
            None
        }
    }
}

fn part_two(data: &str) -> usize {

    let mut data_lines: Vec<&str> = data.lines().collect(); 

    let seed_values: Vec<usize> = data_lines[0].strip_prefix("seeds: ").expect("1").split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut starts: Vec<DataRange> = vec![];

    let mut b = seed_values.into_iter();
    while let Some(aaa) = b.next() {
        starts.push( DataRange {
            start: aaa,
            end: aaa + b.nth(0).unwrap() - 1,
            change: None
        })
    };

    let mut temp_range_list: Vec<DataRange> = vec![];
    let mut range_list: Vec<Vec<DataRange>> = vec![];

    let (_, data_lines) = data_lines.split_at_mut(2);

    for line in data_lines {
        if line.contains("map") { continue; }
        if line.is_empty() {
            temp_range_list.sort();
            range_list.push(temp_range_list);
            temp_range_list = vec![];
            continue;
        }

        let numbers:Vec<usize> = line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
        temp_range_list.push(DataRange { start: numbers[1], end: numbers[1]+numbers[2]-1, change: Some(numbers[0])});
    };

    let mut mins = usize::MAX;
    for ranaa in starts {
        mins = mins.min(get_two_ans(ranaa, 0, &range_list));
    };
    mins
}

fn get_two_ans(current_range: DataRange, range_list_index: usize, range_list: &Vec<Vec<DataRange>>) -> usize {
    let mut mins = usize::MAX;

    
    if range_list_index < range_list.len(){
        for abc in &range_list[range_list_index] {
            let test = abc.overlaps(&current_range);
            if let Some(x) = test {
                let change = abc.change.unwrap();
                let start = x.start - abc.start + change;
                let end = x.end - abc.start + change;
                
                mins = mins.min(get_two_ans(DataRange { start: start, end: end, change: None }, range_list_index+1, range_list));

                if current_range.start < abc.start {
                    mins = mins.min(get_two_ans(DataRange { start: current_range.start, end: x.start-1, change: None }, range_list_index, range_list));
                } else if  current_range.end > abc.end {
                    mins = mins.min(get_two_ans(DataRange { start: x.end+1, end: current_range.end, change: None }, range_list_index, range_list));
                };

            };
        };

        if mins == usize::MAX {
            mins = mins.min(get_two_ans(current_range, range_list_index+1, range_list));
        };
    } else {
        mins = current_range.start;
    }

    mins
}