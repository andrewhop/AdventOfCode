use aoc2025::{day1::day1_part1, input};
use divan::{Bencher, black_box};

fn main() {
    divan::main();
}

#[divan::bench]
fn day1_part1_bench(bencher: Bencher) {
    let input = input("resources/day01_input.txt");

    bencher.bench_local(move || {
        day1_part1(black_box(&input));
    });
}
