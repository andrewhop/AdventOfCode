pub fn day22_part1(input: &[u8]) -> u64 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let mut sum = 0;
    for buyer_seed in ascii_str.lines() {
        let mut next = buyer_seed.parse::<u64>().expect("invalid seed");
        for _ in 0..2000 {
            let temp = next * 64;
            next ^= temp;
            next %= 16777216;
            let temp = next / 32;
            next ^= temp;
            next %= 16777216;
            let temp = next * 2048;
            next ^= temp;
            next %= 16777216;
        }
        sum += next;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input;

    #[test]
    fn part1_tests() {
        let sample = input("resources/day22_sample.txt");
        assert_eq!(day22_part1(&sample), 37327623);

        let sample = input("resources/day22_input.txt");
        assert_eq!(day22_part1(&sample), 14082561342);
    }
}
