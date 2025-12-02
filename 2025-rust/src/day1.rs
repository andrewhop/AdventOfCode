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

pub fn day1_part1_lowlevel(input: &[u8]) -> i64 {
    let mut dial: i64 = 50;
    let mut count: i64 = 0;
    let mut i = 0;
    let len = input.len();

    while i < len {
        // SAFETY: i < len checked above
        let dir = unsafe { *input.get_unchecked(i) };
        i += 1;

        // Parse number inline
        let mut num: i64 = 0;
        while i < len {
            let b = unsafe { *input.get_unchecked(i) };
            if b == b'\n' {
                break;
            }
            num = num * 10 + (b - b'0') as i64;
            i += 1;
        }
        i += 1; // skip newline

        dial = match dir {
            b'L' => (dial - num).rem_euclid(100),
            b'R' => (dial + num).rem_euclid(100),
            _ => dial,
        };

        if dial == 0 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::input;

    use super::*;

    #[test]
    fn day1_part1_sample() {
        let input = input("resources/day01_sample.txt");
        println!("input: {:?}", input);
        assert_eq!(3, day1_part1(&input));
        assert_eq!(3, day1_part1_lowlevel(&input))
    }
    #[test]
    fn day1_part1_test() {
        let input = input("resources/day01_input.txt");
        assert_eq!(984, day1_part1(&input));
        assert_eq!(984, day1_part1_lowlevel(&input));
    }
}
