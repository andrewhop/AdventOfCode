mod day8;

fn input(name: &str) -> Vec<u8> {
    std::fs::read(name).unwrap()
}

pub fn find_first_rust_iter(input: &Vec<u8>, target: u8) -> usize {
    if let Some(index) = input.iter().position(|&x| x == target) {
        return index;
    } else {
        panic!("Could not find {} in vec", target);
    }
}

pub fn find_first_loop(input: &Vec<u8>, target: u8) -> usize {
    let mut index: usize = 0;
    while (index < input.len()) {
        if (input[index] == target) {
            return index;
        }
        index += 1;
    }
    panic!("Could not find {} in vec", target);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_rust_iter() {
        let vec = vec![10, 20, 30, 40, 50];
        let target = 30;
        let index = find_first_rust_iter(&vec, target);
        assert_eq!(index, 2, "Expected index 2, but got {}", index);
    }

    #[test]
    #[should_panic(expected = "Could not find 60 in vec")]
    fn test_find_first_rust_iter_not_found() {
        let vec = vec![10, 20, 30, 40, 50];
        let target = 60;
        find_first_rust_iter(&vec, target); // Should panic
    }

    #[test]
    fn test_find_first_loop() {
        let vec = vec![10, 20, 30, 40, 50];
        let target = 30;
        let index = find_first_loop(&vec, target);
        assert_eq!(index, 2, "Expected index 2, but got {}", index);
    }

    #[test]
    #[should_panic(expected = "Could not find 60 in vec")]
    fn test_find_first_loop_not_found() {
        let vec = vec![10, 20, 30, 40, 50];
        let target = 60;
        find_first_loop(&vec, target); // Should panic
    }
}
