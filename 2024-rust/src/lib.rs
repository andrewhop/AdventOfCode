pub mod day1;
pub mod day2;
pub mod day3;

pub fn input(name: &str) -> Vec<u8> {
    std::fs::read(name).unwrap()
}
