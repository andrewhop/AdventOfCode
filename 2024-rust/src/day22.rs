use std::collections::{HashMap, HashSet};

pub fn day22_part1(input: &[u8]) -> u64 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let mut sum = 0;
    for buyer_seed in ascii_str.lines() {
        let mut next = buyer_seed.parse::<u64>().expect("invalid seed");
        for _ in 0..2000 {
            next = step(next);
        }
        sum += next;
    }
    sum
}

pub fn day22_part2(input: &[u8]) -> i64 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let mut overall_results: HashMap<Vec<i8>, i64> = HashMap::new();
    let mut overall_max: i64 = 0;
    let mut max_seq: Vec<i8> = vec![];

    for buyer_seed in ascii_str.lines() {
        let mut buyer_results: HashSet<Vec<i8>> = HashSet::new();
        let mut change_seq = Vec::new();
        let mut next = buyer_seed.parse::<u64>().expect("invalid seed");
        for _ in 0..2000 {
            let previous_price: i8 = (next % 10) as i8;
            next = step(next);
            let current_price = (next % 10) as i8;
            let price_diff = current_price - previous_price;

            if change_seq.len() == 4 {
                change_seq.remove(0);
            }
            change_seq.push(price_diff);
            if change_seq.len() == 4 {
                // Already added this once, need to check before adding it to the global
                if !buyer_results.contains(&change_seq) {
                    // let current_sequence_buyer_max = *buyer_results.get(&change_seq).unwrap();
                    // // new price is better, remove it from
                    // if current_price as i64 > current_sequence_buyer_max {
                    //     buyer_results.insert(change_seq.clone(), current_price as i64);
                    //     let seq_max = *overall_results.entry(change_seq.clone()).or_default();
                    //     let new_seq_max =
                    //         seq_max + current_price as i64 - current_sequence_buyer_max;
                    //     overall_results.insert(change_seq.clone(), new_seq_max);
                    //     if new_seq_max > overall_max {
                    //         overall_max = new_seq_max;
                    //         max_seq = change_seq.clone();
                    //     }
                    // }
                    buyer_results.insert(change_seq.clone());
                    let seq_max = *overall_results.entry(change_seq.clone()).or_default();
                    let new_seq_max = seq_max + current_price as i64;
                    overall_results.insert(change_seq.clone(), new_seq_max);
                    if new_seq_max > overall_max {
                        overall_max = new_seq_max;
                        max_seq = change_seq.clone();
                    }
                }
            }
        }
    }

    println!("max seq: {:?} = {}", max_seq, overall_max);
    overall_max
}

fn step(input: u64) -> u64 {
    let mut next = input;
    let temp = next * 64;
    next ^= temp;
    next %= 16777216;
    let temp = next / 32;
    next ^= temp;
    next %= 16777216;
    let temp = next * 2048;
    next ^= temp;
    next %= 16777216;
    next
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

    #[test]
    fn part2_tests() {
        let sample = input("resources/day22_sample2.txt");
        assert_eq!(day22_part2(&sample), 23);

        let sample = input("resources/day22_input.txt");
        assert_eq!(day22_part2(&sample), 1568);
    }
}
