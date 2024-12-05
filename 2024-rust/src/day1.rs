use std::collections::{BinaryHeap, HashMap};

pub fn day1_part1_vec(input: &Vec<u8>) -> i64 {
    let mut left_values: Vec<i64> = Vec::with_capacity(1001);
    let mut right_values: Vec<i64> = Vec::with_capacity(1001);
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    for line in ascii_str.lines() {
        let mut words = line.split_whitespace();
        if let (Some(left_num), Some(right_num)) = (words.next(), words.next()) {
            match left_num.parse::<i64>() {
                Ok(num) => left_values.push(num),
                Err(e) => panic!("Failed to convert: {}, {}", left_num, e),
            }
            match right_num.parse::<i64>() {
                Ok(num) => right_values.push(num),
                Err(e) => panic!("Failed to convert: {}, {}", right_num, e),
            }
        } else {
            panic!("Line does not contain exactly two words: {}", line);
        }
    }
    left_values.sort();
    right_values.sort();
    let sum_of_differences: i64 = left_values
        .iter()
        .zip(right_values.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    sum_of_differences
}

pub fn day1_part1_heap(input: &Vec<u8>) -> i64 {
    let mut left_values: BinaryHeap<i64> = BinaryHeap::with_capacity(1001);
    let mut right_values: BinaryHeap<i64> = BinaryHeap::with_capacity(1001);
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    for line in ascii_str.lines() {
        let mut words = line.split_whitespace();
        if let (Some(left_num), Some(right_num)) = (words.next(), words.next()) {
            match left_num.parse::<i64>() {
                Ok(num) => left_values.push(num),
                Err(e) => panic!("Failed to convert: {}, {}", left_num, e),
            }
            match right_num.parse::<i64>() {
                Ok(num) => right_values.push(num),
                Err(e) => panic!("Failed to convert: {}, {}", right_num, e),
            }
        } else {
            panic!("Line does not contain exactly two words: {}", line);
        }
    }
    let mut sum_of_differences = 0;
    while !left_values.is_empty() {
        sum_of_differences += (left_values.pop().unwrap() - right_values.pop().unwrap()).abs();
    }
    sum_of_differences
}

pub fn day1_part1_multi_pass(input: &Vec<u8>) -> i64 {
    let mut left_values: Vec<i64> = Vec::with_capacity(1001);
    let mut right_values: Vec<i64> = Vec::with_capacity(1001);
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    for line in ascii_str.lines() {
        let mut words = line.split_whitespace();
        if let (Some(left_num), Some(right_num)) = (words.next(), words.next()) {
            match left_num.parse::<i64>() {
                Ok(num) => left_values.push(num),
                Err(e) => panic!("Failed to convert: {}, {}", left_num, e),
            }
            match right_num.parse::<i64>() {
                Ok(num) => right_values.push(num),
                Err(e) => panic!("Failed to convert: {}, {}", right_num, e),
            }
        } else {
            panic!("Line does not contain exactly two words: {}", line);
        }
    }
    let mut sum_of_differences = 0;
    while !left_values.is_empty() {
        let (left_index, left_value) = left_values
            .iter()
            .enumerate()
            .fold((0, left_values[0]), |(min_idx, min_val), (idx, &val)| {
                if val < min_val {
                    (idx, val)
                } else {
                    (min_idx, min_val)
                }
            });
        let last_left = left_values.pop().unwrap();
        if left_index < left_values.len() {
            left_values[left_index] = last_left;
        }
        let (right_index, right_value) = right_values
            .iter()
            .enumerate()
            .fold((0, right_values[0]), |(min_idx, min_val), (idx, &val)| {
                if val < min_val {
                    (idx, val)
                } else {
                    (min_idx, min_val)
                }
            });
        let last_right = right_values.pop().unwrap();
        if right_index < right_values.len() {
            right_values[right_index] = last_right;
        }
        sum_of_differences += (left_value - right_value).abs();

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
    fn part1_tests() {
        let sample = input("resources/day1_sample.txt");
        assert_eq!(day1_part1_vec(&sample), 11);
        assert_eq!(day1_part1_multi_pass(&sample), 11);
        assert_eq!(day1_part1_heap(&sample), 11);

        let input = input("resources/day1_input.txt");
        assert_eq!(day1_part1_vec(&input), 2970687);
        assert_eq!(day1_part1_multi_pass(&input), 2970687);
        assert_eq!(day1_part1_heap(&input), 2970687);
    }

    #[test]
    fn part2_tests() {
        let sample = input("resources/day1_sample.txt");
        assert_eq!(day1_part2(&sample), 31);

        let input = input("resources/day1_input.txt");
        assert_eq!(day1_part2(&input), 23963899);
    }
}