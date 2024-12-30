use once_cell::sync::Lazy;
use regex::Regex;

fn day3_part1_regex_core(input: &Vec<u8>, regex: &Regex) -> u32 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let result = regex
        .captures_iter(ascii_str)
        .map(|mul| {
            let left = mul.name("left").unwrap().as_str().parse::<u32>().unwrap();
            let right = mul.name("right").unwrap().as_str().parse::<u32>().unwrap();
            left * right
        })
        .sum();
    result
}

const PART1_REGEX_STRING: &str = r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)";

pub fn day3_part1_regex(input: &Vec<u8>) -> u32 {
    let mul_regex = Regex::new(PART1_REGEX_STRING).unwrap();
    day3_part1_regex_core(input, &mul_regex)
}

static MUL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(PART1_REGEX_STRING).unwrap());

pub fn day3_part1_lazy_regex(input: &Vec<u8>) -> u32 {
    day3_part1_regex_core(input, &MUL_REGEX)
}

const PART2_REGEX_STRING: &str = r"do\(\)|don't\(\)|mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)";
static PART2_MUL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(PART2_REGEX_STRING).unwrap());

pub fn day3_part2_regex(input: &Vec<u8>) -> u32 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let mut mul_on = true;
    let result = PART2_MUL_REGEX
        .captures_iter(ascii_str)
        .map(|mul| {
            if &mul[0] == "do()" {
                mul_on = true;
                0
            } else if &mul[0] == "don't()" {
                mul_on = false;
                return 0;
            } else if mul_on {
                let left = mul.name("left").unwrap().as_str().parse::<u32>().unwrap();
                let right = mul.name("right").unwrap().as_str().parse::<u32>().unwrap();
                return left * right;
            } else {
                return 0;
            }
        })
        .sum();
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input;

    #[test]
    fn part1_tests() {
        assert_eq!(std::mem::size_of::<u16>(), std::mem::size_of::<(u8, u8)>());
        let sample = input("resources/day3_sample.txt");
        assert_eq!(day3_part1_regex(&sample), 161);
        assert_eq!(day3_part1_lazy_regex(&sample), 161);

        let input = input("resources/day3_input.txt");
        assert_eq!(day3_part1_regex(&input), 173785482);
        assert_eq!(day3_part1_lazy_regex(&input), 173785482);
    }

    #[test]
    fn part2_tests() {
        assert_eq!(std::mem::size_of::<u16>(), std::mem::size_of::<(u8, u8)>());
        let sample = input("resources/day3_sample.txt");
        assert_eq!(day3_part2_regex(&sample), 48);

        let input = input("resources/day3_input.txt");
        assert_eq!(day3_part2_regex(&input), 83158140);
    }
}
