use crate::day2::State::{Decrementing, Incrementing, Unknown};
use std::cmp::{Ordering, PartialEq};

#[derive(PartialEq)]
enum State {
    Unknown,
    Decrementing,
    Incrementing,
}

fn check_all_safe(line: &str) -> bool {
    let mut numbers = line
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap());
    let mut prev = numbers.next().unwrap();
    let mut overall_direction = Unknown;
    for number in numbers {
        let (diff, direction) = match prev.cmp(&number) {
            Ordering::Equal => {
                return false;
            }
            Ordering::Less => (number - prev, Decrementing),
            Ordering::Greater => (prev - number, Incrementing),
        };
        if diff > 3 {
            return false;
        }
        if overall_direction == Unknown {
            overall_direction = direction;
        } else {
            if overall_direction != direction {
                return false;
            }
        }
        prev = number;
    }
    true
}

pub fn day2_part1_clean(input: &Vec<u8>) -> u32 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let mut safe = 0;
    for line in ascii_str.lines() {
        if check_all_safe(line) {
            safe += 1;
        }
    }
    safe
}

pub fn day2_part1_gross(input: &Vec<u8>) -> u32 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let mut safe = 0;
    'line_loop: for line in ascii_str.lines() {
        let mut words = line.split_whitespace().into_iter();
        let mut previous: u32 = words.next().unwrap().parse::<u32>().unwrap();
        let mut direction = Unknown;
        for word in words {
            let num = word.parse::<u32>().unwrap();
            if previous == num {
                continue 'line_loop;
            } else if previous > num {
                if direction == Unknown {
                    direction = Decrementing;
                } else if direction == Incrementing {
                    continue 'line_loop;
                }
                let diff = previous - num;
                if diff > 3 {
                    continue 'line_loop;
                }
            } else if previous < num {
                if direction == Unknown {
                    direction = Incrementing;
                } else if direction == Decrementing {
                    continue 'line_loop;
                }
                let diff = num - previous;
                if diff > 3 {
                    continue 'line_loop;
                }
            }
            previous = num;
        }
        safe += 1;
    }
    safe
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input;

    #[test]
    fn part1_tests() {
        assert_eq!(std::mem::size_of::<u16>(), std::mem::size_of::<(u8, u8)>());
        let sample = input("resources/day2_sample.txt");
        assert_eq!(day2_part1_gross(&sample), 2);
        assert_eq!(day2_part1_clean(&sample), 2);

        let input = input("resources/day2_input.txt");
        assert_eq!(day2_part1_clean(&input), 359);
        assert_eq!(day2_part1_gross(&input), 359);
    }
}
