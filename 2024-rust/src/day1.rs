use std::collections::{BinaryHeap, HashMap};

pub fn day1_part1(input: &Vec<u8>) -> i64 {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    for line in ascii_str.lines() {
        let mut words = line.split_whitespace();
        if let (Some(left_num), Some(right_num)) = (words.next(), words.next()) {
            match left_num.parse::<i64>() {
                Ok(num) => left.push(num),
                Err(e) => panic!("Failed to convert: {}, {}", left_num, e),
            }
            match right_num.parse::<i64>() {
                Ok(num) => right.push(num),
                Err(e) => panic!("Failed to convert: {}, {}", right_num, e),
            }
        } else {
            panic!("Line does not contain exactly two words: {}", line);
        }
    }
    left.sort();
    right.sort();
    let sum_of_differences: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    sum_of_differences
}

pub fn day1_part1_heap(input: &Vec<u8>) -> i64 {
    let mut left: BinaryHeap<i64> = BinaryHeap::new();
    let mut right: BinaryHeap<i64> = BinaryHeap::new();
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    for line in ascii_str.lines() {
        let mut words = line.split_whitespace();
        if let (Some(left_num), Some(right_num)) = (words.next(), words.next()) {
            match left_num.parse::<i64>() {
                Ok(num) => left.push(num),
                Err(e) => panic!("Failed to convert: {}, {}", left_num, e),
            }
            match right_num.parse::<i64>() {
                Ok(num) => right.push(num),
                Err(e) => panic!("Failed to convert: {}, {}", right_num, e),
            }
        } else {
            panic!("Line does not contain exactly two words: {}", line);
        }
    }
    let mut sum_of_differences = 0;
    while !left.is_empty() {
        sum_of_differences += (left.pop().unwrap() - right.pop().unwrap()).abs();
    }
    sum_of_differences
}

pub fn day1_part2(input: &Vec<u8>) -> i64 {
    let mut left: Vec<i64> = Vec::new();
    let mut right_map: HashMap<i64, u16> = HashMap::new();
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    for line in ascii_str.lines() {
        let mut words = line.split_whitespace();
        if let (Some(left_num), Some(right_num)) = (words.next(), words.next()) {
            match left_num.parse::<i64>() {
                Ok(num) => left.push(num),
                Err(e) => panic!("Failed to convert: {}, {}", left_num, e),
            }
            match right_num.parse::<i64>() {
                Ok(num) => {
                    *right_map.entry(num).or_insert(0) += 1;
                }
                Err(e) => panic!("Failed to convert: {}, {}", right_num, e),
            }
        } else {
            panic!("Line does not contain exactly two words: {}", line);
        }
    }

    let mut simalarity = 0;
    for entry in left.iter() {
        let count = *right_map.get(entry).unwrap_or(&0);
        simalarity += entry * count as i64;
    }
    simalarity
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input;

    #[test]
    fn p1_sample() {
        let input = input("resources/day1_sample.txt");
        let result = day1_part1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn p1() {
        let input = input("resources/day1_input.txt");
        let result = day1_part1(&input);
        assert_eq!(result, 2970687);
    }

    #[test]
    fn p1_heap_sample() {
        let input = input("resources/day1_sample.txt");
        let result = day1_part1_heap(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn p1_heap() {
        let input = input("resources/day1_input.txt");
        let result = day1_part1_heap(&input);
        assert_eq!(result, 2970687);
    }

    #[test]
    fn p2_sample() {
        let input = input("resources/day1_sample.txt");
        let result = day1_part2(&input);
        assert_eq!(result, 31);
    }

    #[test]
    fn p2() {
        let input = input("resources/day1_input.txt");
        let result = day1_part2(&input);
        assert_eq!(result, 23963899);
    }
}
