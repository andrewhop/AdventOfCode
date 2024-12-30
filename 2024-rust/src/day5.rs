use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn is_update_valid(line: &str, rules: &mut HashMap<u32, Vec<u32>>) -> Option<u32> {
    let pages_to_print = line.split(",");
    let mut printed_pages = HashSet::new();
    for page in pages_to_print {
        let page_number = page.parse::<u32>().unwrap();
        printed_pages.insert(page_number);
        let required_printed = rules.entry(page_number).or_default();
        for required in required_printed {
            if printed_pages.contains(required) {
                return None;
            }
        }
    }

    let line_length = line.len();
    let beginning = line_length / 2 - 1;

    let raw_num = &line[beginning..beginning + 2];
    let number = raw_num.parse::<u32>().unwrap();
    Some(number)
}

pub fn day5_part1(input: &Vec<u8>) -> u32 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let lines = ascii_str.lines();

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut parsing_rules = true;
    let mut sum = 0;
    for line in lines {
        if line.is_empty() {
            parsing_rules = false;
        } else if parsing_rules {
            let before = line[0..2].parse::<u32>().unwrap();
            let after = line[3..5].parse::<u32>().unwrap();
            rules.entry(before).or_default().push(after);
        } else if let Some(x) = is_update_valid(line, &mut rules) { sum += x }
    }
    sum
}

fn fix_line(line: &str, rules: &mut HashMap<u32, Vec<u32>>) -> u32 {
    let mut pages_to_print = line
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    pages_to_print.sort_by(|a, b| {
        let page_rules = rules.entry(*a).or_default();
        if page_rules.contains(b) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    pages_to_print[pages_to_print.len() / 2]
}
pub fn day5_part2(input: &Vec<u8>) -> u32 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let lines = ascii_str.lines();

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut parsing_rules = true;
    let mut sum = 0;
    for line in lines {
        if line.is_empty() {
            parsing_rules = false;
        } else if parsing_rules {
            let before = line[0..2].parse::<u32>().unwrap();
            let after = line[3..5].parse::<u32>().unwrap();
            rules.entry(before).or_default().push(after);
        } else {
            match is_update_valid(line, &mut rules) {
                Some(_) => {}
                None => {
                    sum += fix_line(line, &mut rules);
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input;

    #[test]
    fn part1_tests() {
        let sample = input("resources/day5_sample.txt");
        assert_eq!(day5_part1(&sample), 143);

        let sample = input("resources/day5_input.txt");
        assert_eq!(day5_part1(&sample), 5268);
    }

    #[test]
    fn part2_tests() {
        let sample = input("resources/day5_sample.txt");
        assert_eq!(day5_part2(&sample), 123);

        let sample = input("resources/day5_input.txt");
        assert_eq!(day5_part2(&sample), 5799);
    }
}
