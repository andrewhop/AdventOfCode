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

#[inline(never)]
pub fn day1_part1_unsafe(input: &[u8]) -> i64 {
    // Full table: covers index 0 to 2099 (values -1000 to +1099 after offset)
    const MOD100_FULL: [i32; 2100] = {
        let mut table = [0i32; 2100];
        let mut i = 0i32;
        while i < 2100 {
            table[i as usize] = (i - 1000).rem_euclid(100);
            i += 1;
        }
        table
    };

    unsafe {
        let mut ptr = input.as_ptr();
        let end = ptr.add(input.len());
        let mut dial: i32 = 50;
        let mut count: i64 = 0;

        while ptr < end {
            let dir = *ptr;
            ptr = ptr.add(1);

            // Parse number - unrolled for 1-3 digit case
            let d1 = (*ptr - b'0') as i32;
            ptr = ptr.add(1);

            let mut num = d1;
            if ptr < end && *ptr != b'\n' {
                let d2 = (*ptr - b'0') as i32;
                ptr = ptr.add(1);
                num = d1 * 10 + d2;

                if ptr < end && *ptr != b'\n' {
                    let d3 = (*ptr - b'0') as i32;
                    ptr = ptr.add(1);
                    num = num * 10 + d3;
                }
            }

            // Skip newline if present
            if ptr < end {
                ptr = ptr.add(1);
            }

            // No modulo needed! Full table lookup
            // sign: L=-1, R=+1
            let sign = 1 - 2 * (((dir >> 3) & 1) as i32);
            dial = *MOD100_FULL.get_unchecked((dial + sign * num + 1000) as usize);

            count += (dial == 0) as i64;
        }

        count
    }
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
        assert_eq!(3, day1_part1_lowlevel(&input));
        assert_eq!(3, day1_part1_unsafe(&input));
    }
    #[test]
    fn day1_part1_test() {
        let input = input("resources/day01_input.txt");
        assert_eq!(984, day1_part1(&input));
        assert_eq!(984, day1_part1_lowlevel(&input));
        assert_eq!(984, day1_part1_unsafe(&input));
    }
}
