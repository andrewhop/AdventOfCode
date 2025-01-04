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

const MAX_VALUE: usize = usize::pow(19, 4);
fn seq_to_num(seq: &[i8]) -> usize {
    let mut result = (seq[0] + 9) as usize;
    result += (seq[1] + 9) as usize * 19;
    result += (seq[2] + 9) as usize * usize::pow(19, 2);
    result += (seq[3] + 9) as usize * usize::pow(19, 3);
    result
}

trait Mapish {
    fn get_or_default(&mut self, key: Vec<i8>) -> &u16;
    fn insert(&mut self, key: Vec<i8>, value: u16);
}

impl Mapish for HashMap<Vec<i8>, u16> {
    fn get_or_default(&mut self, key: Vec<i8>) -> &u16 {
        self.entry(key).or_default()
    }
    fn insert(&mut self, key: Vec<i8>, value: u16) {
        self.insert(key, value);
    }
}

trait Setish {
    fn contains(&self, key: &[i8]) -> bool;
    fn insert(&mut self, key: Vec<i8>);
}

impl Setish for HashSet<Vec<i8>> {
    fn contains(&self, key: &[i8]) -> bool {
        self.contains(key)
    }
    fn insert(&mut self, key: Vec<i8>) {
        self.insert(key);
    }
}

struct ArrayMap {
    values: Vec<u16>,
}

impl Default for ArrayMap {
    fn default() -> Self {
        ArrayMap {
            values: vec![0; MAX_VALUE],
        }
    }
}

impl Mapish for ArrayMap {
    fn get_or_default(&mut self, key: Vec<i8>) -> &u16 {
        let index = seq_to_num(&key);
        &self.values[index]
    }

    fn insert(&mut self, key: Vec<i8>, value: u16) {
        let index = seq_to_num(&key);
        self.values[index] = value;
    }
}

struct ArraySet {
    values: Vec<bool>,
}

impl Default for ArraySet {
    fn default() -> Self {
        ArraySet {
            values: vec![false; MAX_VALUE],
        }
    }
}

impl Setish for ArraySet {
    fn contains(&self, key: &[i8]) -> bool {
        self.values[seq_to_num(key)]
    }
    fn insert(&mut self, key: Vec<i8>) {
        self.values[seq_to_num(&key)] = true;
    }
}

fn day22_part2_core<M, S>(input: &[u8]) -> u16
where
    M: Mapish + Default,
    S: Setish + Default,
{
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let mut overall_results = M::default();
    let mut overall_max: u16 = 0;

    for buyer_seed in ascii_str.lines() {
        let mut buyer_results = S::default();
        let mut change_seq = Vec::new();
        let mut next = buyer_seed.parse::<u64>().expect("invalid seed");
        for _ in 0..2_000 {
            let previous_price: i8 = (next % 10) as i8;
            next = step(next);
            let current_price = (next % 10) as i8;
            let price_diff = current_price - previous_price;

            if change_seq.len() == 4 {
                change_seq.remove(0);
            }
            change_seq.push(price_diff);
            if change_seq.len() == 4 && !buyer_results.contains(&change_seq) {
                buyer_results.insert(change_seq.clone());
                let seq_max = *overall_results.get_or_default(change_seq.clone());
                let new_seq_max = seq_max + current_price as u16;
                overall_results.insert(change_seq.clone(), new_seq_max);
                if new_seq_max > overall_max {
                    overall_max = new_seq_max;
                }
            }
        }
    }
    overall_max
}

pub fn day22_part2_std(input: &[u8]) -> u16 {
    day22_part2_core::<HashMap<Vec<i8>, u16>, HashSet<Vec<i8>>>(input)
}

pub fn day22_part2_arrayish(input: &[u8]) -> u16 {
    day22_part2_core::<ArrayMap, ArraySet>(input)
}

pub fn step(input: u64) -> u64 {
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

const PRUNE_MASK: u64 = 16777216;
pub fn step_shift(input: u64) -> u64 {
    let mut next = input;
    next ^= (next << 6) & PRUNE_MASK;
    next ^= next >> 5;
    next ^= (next << 11) & PRUNE_MASK;
    next
}

pub fn day22_part2(input: &[u8]) -> u16 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let mut overall_results: HashMap<Vec<i8>, u16> = HashMap::new();
    let mut overall_max: u16 = 0;

    for buyer_seed in ascii_str.lines() {
        let mut buyer_results: HashSet<Vec<i8>> = HashSet::new();
        let mut change_seq = Vec::new();
        let mut next = buyer_seed.parse::<u64>().expect("invalid seed");
        for _ in 0..2_000 {
            let previous_price: i8 = (next % 10) as i8;
            next = step(next);
            let current_price = (next % 10) as i8;
            let price_diff = current_price - previous_price;

            if change_seq.len() == 4 {
                change_seq.remove(0);
            }
            change_seq.push(price_diff);
            if change_seq.len() == 4 && !buyer_results.contains(&change_seq) {
                buyer_results.insert(change_seq.clone());
                let seq_max = *overall_results.entry(change_seq.clone()).or_default();
                let new_seq_max = seq_max + current_price as u16;
                overall_results.insert(change_seq.clone(), new_seq_max);
                if new_seq_max > overall_max {
                    overall_max = new_seq_max;
                }
            }
        }
    }
    overall_max
}

struct SequenceNum {
    first: i8,
    second: i8,
    third: i8,
    fourth: i8,
    count: usize,
}

impl SequenceNum {
    fn new() -> SequenceNum {
        SequenceNum {
            first: 0,
            second: 0,
            third: 0,
            fourth: 0,
            count: 0,
        }
    }
    fn push(&mut self, new: i8) {
        self.fourth = self.third;
        self.third = self.second;
        self.second = self.first;
        self.first = new + 9;
        self.count += 1;
    }
    fn to_key(&self) -> usize {
        let mut result = self.first as usize;
        result += self.second as usize * 19;
        result += self.third as usize * usize::pow(19, 2);
        result += self.fourth as usize * usize::pow(19, 3);
        result
    }
}

pub fn day22_part2_array_native(input: &[u8]) -> u16 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");

    let mut overall_results: Vec<u16> = vec![0; MAX_VALUE];
    let mut overall_max: u16 = 0;
    let mut buyer_results: Vec<u16> = vec![0; MAX_VALUE];

    for (id, buyer_seed) in ascii_str.lines().enumerate() {
        let buyer_id = (id + 1) as u16;
        let mut change_seq = SequenceNum::new();
        let mut next = buyer_seed.parse::<u64>().expect("invalid seed");
        for _ in 0..2_000 {
            let previous_price: i8 = (next % 10) as i8;
            next = step(next);
            let current_price = (next % 10) as i8;
            let price_diff = current_price - previous_price;
            change_seq.push(price_diff);
            if change_seq.count >= 4 {
                let sequence_key = change_seq.to_key();
                if buyer_results[sequence_key] != buyer_id {
                    buyer_results[sequence_key] = buyer_id;
                    let mut temp = overall_results[sequence_key];
                    temp += current_price as u16;
                    overall_results[sequence_key] = temp;
                    if temp > overall_max {
                        overall_max = temp;
                    }
                }
            }
        }
    }
    overall_max
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
    fn mapish_test() {
        let mut hashmap: HashMap<Vec<i8>, u16> = HashMap::new();
        let mut arraymap = ArrayMap::default();

        let mut key = vec![1, 2, 3, 9];
        assert_eq!(
            hashmap.get_or_default(key.clone()),
            arraymap.get_or_default(key.clone())
        );
        assert_eq!(
            hashmap.get_or_default(key.clone()),
            arraymap.get_or_default(key.clone())
        );
        hashmap.insert(key.clone(), 10);
        arraymap.insert(key.clone(), 10);
        assert_eq!(
            hashmap.get_or_default(key.clone()),
            arraymap.get_or_default(key.clone())
        );

        key[0] = -9;
        assert_eq!(
            hashmap.get_or_default(key.clone()),
            arraymap.get_or_default(key.clone())
        );
        hashmap.insert(key.clone(), 20);
        arraymap.insert(key.clone(), 20);
        assert_eq!(
            hashmap.get_or_default(key.clone()),
            arraymap.get_or_default(key.clone())
        );
    }

    #[test]
    fn mapish_exhaustive_test() {
        let mut hashmap: HashMap<Vec<i8>, u16> = HashMap::new();
        let mut arraymap = ArrayMap::default();
        let mut key = vec![0, 0, 0, 0];
        for first in -9..9 {
            key[3] = first;
            for second in -9..9 {
                key[2] = second;
                for third in -9..9 {
                    key[1] = third;
                    for fourth in -9..9 {
                        key[0] = fourth;
                        assert_eq!(
                            hashmap.get_or_default(key.clone()),
                            arraymap.get_or_default(key.clone())
                        );
                    }
                }
            }
        }
        let mut count: u32 = 1;
        for first in -9..9 {
            key[3] = first;
            for second in -9..9 {
                key[2] = second;
                for third in -9..9 {
                    key[1] = third;
                    for fourth in -9..9 {
                        key[0] = fourth;
                        hashmap.insert(key.clone(), (count % 65536) as u16);
                        arraymap.insert(key.clone(), (count % 65536) as u16);
                        count += 1;
                    }
                }
            }
        }
        count = 1;
        for first in -9..9 {
            key[3] = first;
            for second in -9..9 {
                key[2] = second;
                for third in -9..9 {
                    key[1] = third;
                    for fourth in -9..9 {
                        key[0] = fourth;

                        assert_eq!(*hashmap.get_or_default(key.clone()), (count % 65536) as u16);
                        assert_eq!(
                            *arraymap.get_or_default(key.clone()),
                            (count % 65536) as u16,
                            "Failed at count {}, key {:?}",
                            count,
                            key
                        );
                        count += 1;
                    }
                }
            }
        }
    }

    #[test]
    fn setish_test() {
        let mut hashset: HashSet<Vec<i8>> = HashSet::new();
        let mut arrayset = ArraySet::default();

        let mut key = vec![1, 2, 3, 4];
        assert_eq!(hashset.contains(&key), arrayset.contains(&key));
        assert_eq!(hashset.contains(&key), arrayset.contains(&key));
        hashset.insert(key.clone());
        arrayset.insert(key.clone());
        assert_eq!(hashset.contains(&key), arrayset.contains(&key));
        key[0] = -9;
        assert_eq!(hashset.contains(&key), arrayset.contains(&key));
        hashset.insert(key.clone());
        arrayset.insert(key.clone());
        assert_eq!(hashset.contains(&key), arrayset.contains(&key));
    }

    #[test]
    fn part2_tests() {
        let sample = input("resources/day22_sample2.txt");
        assert_eq!(day22_part2(&sample), 23);
        assert_eq!(day22_part2_std(&sample), 23);
        assert_eq!(day22_part2_arrayish(&sample), 23);
        assert_eq!(day22_part2_array_native(&sample), 23);

        let input = input("resources/day22_input.txt");
        // assert_eq!(day22_part2(&input), 1568);
        // assert_eq!(day22_part2_std(&input), 1568);
        assert_eq!(day22_part2_arrayish(&input), 1568);
        assert_eq!(day22_part2_array_native(&input), 1568);
    }
}
