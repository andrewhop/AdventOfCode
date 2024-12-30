use std::collections::{BinaryHeap, HashMap};

trait Pushable {
    fn push(&mut self, item: u32);
}
impl Pushable for Vec<u32> {
    fn push(&mut self, item: u32) {
        self.push(item);
    }
}
impl Pushable for BinaryHeap<u32> {
    fn push(&mut self, item: u32) {
        self.push(item);
    }
}
fn convert_string_to_vecs<T: Pushable>(input: &[u8], left_values: &mut T, right_values: &mut T) {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    for line in ascii_str.lines() {
        let mut words = line.split_whitespace();
        let left = words.next().unwrap().parse::<u32>().unwrap();
        left_values.push(left);
        let right = words.next().unwrap().parse::<u32>().unwrap();
        right_values.push(right);
    }
}

pub fn day1_part1_vec(input: &[u8]) -> u32 {
    let mut left_values: Vec<u32> = Vec::with_capacity(1001);
    let mut right_values: Vec<u32> = Vec::with_capacity(1001);
    convert_string_to_vecs(input, &mut left_values, &mut right_values);
    left_values.sort_unstable();
    right_values.sort_unstable();
    let sum_of_differences: u32 = left_values
        .iter()
        .zip(right_values.iter())
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .sum();
    sum_of_differences
}

pub fn day1_part1_radix(input: &[u8]) -> u32 {
    let mut left_values: Vec<u32> = Vec::with_capacity(1001);
    let mut right_values: Vec<u32> = Vec::with_capacity(1001);
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let mut max_value = 0;
    for line in ascii_str.lines() {
        let mut words = line.split_whitespace();
        let left = words.next().unwrap().parse::<u32>().unwrap();
        left_values.push(left);
        if left > max_value {
            max_value = left;
        }
        let right = words.next().unwrap().parse::<u32>().unwrap();
        right_values.push(right);
        if right > max_value {
            max_value = right;
        }
    }
    let num_pairs = left_values.len();
    let num_buckets = max_value + 1;
    let mut buckets: Vec<(u8, u8)> = vec![(0, 0); num_buckets as usize];

    left_values.into_iter().for_each(|index| {
        buckets[index as usize].0 += 1;
    });

    right_values.into_iter().for_each(|index| {
        buckets[index as usize].1 += 1;
    });

    let mut left_index: usize = 0;
    let mut right_index: usize = 0;
    let mut sum_of_differences = 0;

    let mut pairs = 0;
    while pairs < num_pairs {
        pairs += 1;
        while buckets[left_index].0 == 0 {
            left_index += 1;
        }
        buckets[left_index].0 -= 1;

        while buckets[right_index].1 == 0 {
            right_index += 1;
        }
        buckets[right_index].1 -= 1;

        sum_of_differences += if left_index < right_index {
            right_index - left_index
        } else {
            left_index - right_index
        };
    }

    sum_of_differences as u32
}

pub fn day1_part1_radix_one_pass(input: &[u8]) -> u32 {
    let mut buckets: Vec<(u8, u8)> = vec![(0, 0); 100_000_usize];
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let mut num_pairs = 0;
    for line in ascii_str.lines() {
        let mut words = line.split_whitespace();
        let left = words.next().unwrap().parse::<u32>().unwrap();
        buckets[left as usize].0 += 1;
        let right = words.next().unwrap().parse::<u32>().unwrap();
        buckets[right as usize].1 += 1;
        num_pairs += 1;
    }

    let mut left_index: usize = 0;
    let mut right_index: usize = 0;
    let mut sum_of_differences = 0;

    let mut pairs = 0;
    while pairs < num_pairs {
        pairs += 1;
        while buckets[left_index].0 == 0 {
            left_index += 1;
        }
        buckets[left_index].0 -= 1;

        while buckets[right_index].1 == 0 {
            right_index += 1;
        }
        buckets[right_index].1 -= 1;

        sum_of_differences += if left_index < right_index {
            right_index - left_index
        } else {
            left_index - right_index
        };
    }

    sum_of_differences as u32
}

pub fn day1_part1_heap(input: &[u8]) -> u32 {
    let mut left_values: BinaryHeap<u32> = BinaryHeap::with_capacity(1001);
    let mut right_values: BinaryHeap<u32> = BinaryHeap::with_capacity(1001);
    convert_string_to_vecs(input, &mut left_values, &mut right_values);
    let mut sum_of_differences = 0;
    while !left_values.is_empty() {
        let left = left_values.pop().unwrap();
        let right = right_values.pop().unwrap();
        sum_of_differences += if left > right {
            left - right
        } else {
            right - left
        };
    }
    sum_of_differences
}

pub fn day1_part1_multi_pass_fold(input: &[u8]) -> u32 {
    let mut left_values: Vec<u32> = Vec::with_capacity(1001);
    let mut right_values: Vec<u32> = Vec::with_capacity(1001);
    convert_string_to_vecs(input, &mut left_values, &mut right_values);
    let mut sum_of_differences = 0;
    while !left_values.is_empty() {
        let (left_index, left_value) = left_values.iter().enumerate().fold(
            (0, left_values[0]),
            |(min_idx, min_val), (idx, &val)| {
                if val < min_val {
                    (idx, val)
                } else {
                    (min_idx, min_val)
                }
            },
        );
        let last_left = left_values.pop().unwrap();
        if left_index < left_values.len() {
            left_values[left_index] = last_left;
        }
        let (right_index, right_value) = right_values.iter().enumerate().fold(
            (0, right_values[0]),
            |(min_idx, min_val), (idx, &val)| {
                if val < min_val {
                    (idx, val)
                } else {
                    (min_idx, min_val)
                }
            },
        );
        let last_right = right_values.pop().unwrap();
        if right_index < right_values.len() {
            right_values[right_index] = last_right;
        }
        sum_of_differences += if left_value > right_value {
            left_value - right_value
        } else {
            right_value - left_value
        };
    }
    sum_of_differences
}

pub fn day1_part1_multi_pass_loop(input: &[u8]) -> u32 {
    let mut left_values: Vec<u32> = Vec::with_capacity(1001);
    let mut right_values: Vec<u32> = Vec::with_capacity(1001);
    convert_string_to_vecs(input, &mut left_values, &mut right_values);
    let mut sum_of_differences = 0;
    while !left_values.is_empty() {
        let mut left_index = 0;
        let mut left_value = left_values[left_index];
        for (index, value) in left_values.iter().enumerate() {
            if *value < left_value {
                left_value = *value;
                left_index = index;
            }
        }
        let last_left = left_values.pop().unwrap();
        if left_index < left_values.len() {
            left_values[left_index] = last_left;
        }
        let mut right_index = 0;
        let mut right_value = right_values[right_index];
        for (index, value) in right_values.iter().enumerate() {
            if *value < right_value {
                right_value = *value;
                right_index = index;
            }
        }
        let last_right = right_values.pop().unwrap();
        if right_index < right_values.len() {
            right_values[right_index] = last_right;
        }
        sum_of_differences += if left_value > right_value {
            left_value - right_value
        } else {
            right_value - left_value
        };
    }
    sum_of_differences
}

pub fn day1_part2(input: &[u8]) -> u32 {
    let mut left_values: Vec<u32> = Vec::new();
    let mut right_map: HashMap<u32, u16> = HashMap::new();
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    for line in ascii_str.lines() {
        let mut words = line.split_whitespace();
        let left = words.next().unwrap().parse::<u32>().unwrap();
        left_values.push(left);
        let right = words.next().unwrap().parse::<u32>().unwrap();
        *right_map.entry(right).or_insert(0) += 1;
    }

    let mut simalarity = 0;
    for entry in left_values.iter() {
        let count = *right_map.get(entry).unwrap_or(&0);
        simalarity += entry * count as u32;
    }
    simalarity
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input;

    #[test]
    fn part1_tests() {
        assert_eq!(std::mem::size_of::<u16>(), std::mem::size_of::<(u8, u8)>());
        let sample = input("resources/day1_sample.txt");
        assert_eq!(day1_part1_vec(&sample), 11);
        assert_eq!(day1_part1_radix(&sample), 11);
        assert_eq!(day1_part1_radix_one_pass(&sample), 11);
        assert_eq!(day1_part1_multi_pass_fold(&sample), 11);
        assert_eq!(day1_part1_multi_pass_loop(&sample), 11);
        assert_eq!(day1_part1_heap(&sample), 11);

        let input = input("resources/day1_input.txt");
        assert_eq!(day1_part1_vec(&input), 2970687);
        assert_eq!(day1_part1_radix(&input), 2970687);
        assert_eq!(day1_part1_radix_one_pass(&input), 2970687);
        assert_eq!(day1_part1_multi_pass_fold(&input), 2970687);
        assert_eq!(day1_part1_multi_pass_loop(&input), 2970687);
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
