use aoc2023::{find_first_loop, find_first_rust_iter};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
fn find_in_vec_bench(c: &mut Criterion) {
    let target = 255;
    let mut vec: Vec<u8> = (0..100_000)
        .map(|_| {
            let mut value = rand::random::<u8>();
            while value == target {
                value = rand::random::<u8>();
            }
            value
        })
        .collect();

    vec[95_000] = target;

    c.bench_function("find_first_rust_iter", |b| { b.iter(|| find_first_rust_iter(black_box(&vec), black_box(target))) });
    c.bench_function("find_first_loop", |b| { b.iter(|| find_first_loop(black_box(&vec), black_box(target))) });
}

criterion_group!(benches, find_in_vec_bench);
criterion_main!(benches);