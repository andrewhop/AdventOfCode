pub fn day1_part1(input: &[u8]) -> i64 {
    let mut dial: i64 = 50;
    input
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let num: i64 = unsafe { std::str::from_utf8_unchecked(&line[1..]) }
                .parse()
                .expect("not a number");

            dial = match line[0] {
                b'L' => (dial - num).rem_euclid(100),
                b'R' => (dial + num).rem_euclid(100),
                _ => panic!("Invalid direction: {:?}", line[0] as char),
            };

            dial == 0
        })
        .count() as i64
}

#[cfg(test)]
mod tests {
    use crate::input;

    use super::*;

    #[test]
    fn day1_part1_sample() {
        let input = input("resources/day01_sample.txt");
        println!("input: {:?}", input);
        let result = day1_part1(&input);
        assert_eq!(3, result);
    }
    #[test]
    fn day1_part1_test() {
        let input = input("resources/day01_input.txt");
        let result = day1_part1(&input);
        assert_eq!(984, result);
    }
}
