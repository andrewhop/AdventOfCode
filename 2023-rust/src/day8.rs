const MAP_SIZE: u16 = (25 << 10) + (25 << 5) + 25 + 1;

fn word_to_num(input: &[u8]) -> u16 {
    (((input[0] - b'A') as u16) << 10) + (((input[1] - b'A') as u16) << 5) + ((input[2] - b'A') as u16)
}

pub fn day8_part1_low_level(input: &Vec<u8>) -> u64 {
    let end_of_instructions = input.iter().position(|&x| x == b'\n').unwrap();

    let instructions = &input[..end_of_instructions];
    let mut map: [[u16; 2]; MAP_SIZE as usize] = [[0; 2]; MAP_SIZE as usize];

    let mut index = end_of_instructions + 2;
    while index < input.len() {
        let source = word_to_num(&input[index..index+3]);
        let left = word_to_num(&input[index+7..index+10]);
        let right = word_to_num(&input[index+12..index+15]);
        map[source as usize][0] = left;
        map[source as usize][1] = right;
        index+=17;
    }

    let mut instruction_count = 0;
    let mut position = 0;
    let target = word_to_num(&[b'Z', b'Z', b'Z']);

    while position != target {
        let direction = if instructions[instruction_count % instructions.len()] == b'L' {0} else {1};
        instruction_count = instruction_count + 1;
        position = map[position as usize][direction as usize];
    }

    instruction_count as u64
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input;

    #[test]
    fn p1_sample() {
        let input = input("resources/day8_sample.txt");
        let result = day8_part1_low_level(&input);
        assert_eq!(result, 6);
    }

    #[test]
    fn p1() {
        let input = input("resources/day8_input.txt");
        let result = day8_part1_low_level(&input);
        assert_eq!(result, 14429);
    }

    #[test]
    fn word_to_num_test() {
        let a = [b'A', b'A', b'A', b'Z', b'Z', b'Z'];
        assert_eq!(word_to_num(&a[0..3]), 0);
        assert_eq!(word_to_num(&a[1..4]), 25);
        assert_eq!(word_to_num(&a[3..6]), 26425);
        assert_eq!(word_to_num(&a[3..6]) + 1, MAP_SIZE);
    }
}
