#![feature(portable_simd)]
#![feature(array_chunks)]
#![feature(iter_array_chunks)]
pub mod day1;
pub mod day2;
pub mod day22;
pub mod day3;
pub mod day4;
pub mod day5;

pub fn input(name: &str) -> Vec<u8> {
    std::fs::read(name).unwrap()
}
