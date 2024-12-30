const TARGET: &str = "XMAS";

fn is_xmas(grid: &Vec<&str>, start_col: i32, start_row: i32, col_dir: i32, row_dir: i32) -> bool {
    for i in 0..TARGET.len() {
        let letter = TARGET.as_bytes()[i];
        let column = start_col + i as i32 * col_dir;
        let row = start_row + i as i32 * row_dir;
        if column < 0 || row < 0 {
            return false;
        }
        if column > grid[0].len() as i32 - 1 || row > grid.len() as i32 - 1 {
            return false;
        }
        if grid[row as usize].as_bytes()[column as usize] != letter {
            return false;
        }
    }
    true
}

pub fn day4_part1(input: &Vec<u8>) -> u32 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let grid: Vec<_> = ascii_str.lines().collect();
    let mut xmas_count = 0;
    for column in 0..grid[0].len() {
        for row in 0..grid.len() {
            for (col_dir, row_dir) in [
                (0, 1),
                (0, -1),
                (1, 0),
                (1, 1),
                (1, -1),
                (-1, 0),
                (-1, 1),
                (-1, -1),
            ] {
                if is_xmas(&grid, column as i32, row as i32, col_dir, row_dir) {
                    xmas_count += 1;
                }
            }
        }
    }
    xmas_count
}

const X_MAS_MAS: [[&str; 3]; 4] = [
    ["M*M", "*A*", "S*S"],
    ["M*S", "*A*", "M*S"],
    ["S*M", "*A*", "S*M"],
    ["S*S", "*A*", "M*M"],
];

fn check_mas(mas: &[&str; 3], grid: &Vec<&str>, start_col: i32, start_row: i32) -> bool {
    for mas_row in 0..mas.len() {
        for mas_col in 0..mas[mas_row].len() {
            let row = start_row + mas_row as i32;
            let col = start_col + mas_col as i32;
            if row < 0 || col < 0 {
                return false;
            }
            if row >= grid.len() as i32 || col >= grid[0].len() as i32 {
                return false;
            }
            let grid_val = grid[row as usize].as_bytes()[col as usize];
            let mas_val = mas[mas_row].as_bytes()[mas_col];
            if mas_val != b'*' && grid_val != mas_val {
                return false;
            }
        }
    }

    true
}

fn is_mas_mas(grid: &Vec<&str>, start_col: i32, start_row: i32) -> bool {
    for mas in X_MAS_MAS.iter() {
        if check_mas(mas, grid, start_col, start_row) {
            return true;
        }
    }
    return false;
}

pub fn day4_part2(input: &Vec<u8>) -> u32 {
    let ascii_str = std::str::from_utf8(input).expect("input was not UTF8 string");
    let grid: Vec<_> = ascii_str.lines().collect();
    let mut xmas_count = 0;
    for column in 0..grid[0].len() {
        for row in 0..grid.len() {
            if is_mas_mas(&grid, column as i32, row as i32) {
                xmas_count += 1;
            }
        }
    }
    xmas_count
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input;

    #[test]
    fn part1_tests() {
        let sample = input("resources/day4_sample.txt");
        assert_eq!(day4_part1(&sample), 18);

        let sample = input("resources/day4_input.txt");
        assert_eq!(day4_part1(&sample), 2573);
    }

    #[test]
    fn part2_tests() {
        let sample = input("resources/day4_sample.txt");
        assert_eq!(day4_part2(&sample), 9);

        let sample = input("resources/day4_input.txt");
        assert_eq!(day4_part2(&sample), 1850);
    }
}
