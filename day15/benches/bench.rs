#![allow(dead_code)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/main.rs"]
mod main;

fn bench_main(c: &mut Criterion) {
    c.bench_function("parse input (sample)", |b| {
        let file = main::read_file("sample");
        b.iter(|| main::parse_input(black_box(&file)))
    });

    c.bench_function("part 1 (sample)", |b| {
        let input = main::parse_input(&main::read_file("sample"));
        b.iter(|| main::part1(black_box(&input), 10))
    });

    c.bench_function("part 2 (sample)", |b| {
        let input = main::parse_input(&main::read_file("sample"));
        b.iter(|| main::part2(black_box(&input), 20, 20))
    });

    c.bench_function("parse input (real)", |b| {
        let file = main::read_file("input");
        b.iter(|| main::parse_input(black_box(&file)))
    });

    c.bench_function("part 1 (real)", |b| {
        let input = main::parse_input(&main::read_file("input"));
        b.iter(|| main::part1(black_box(&input), 2_000_000))
    });
    
    c.bench_function("part 2 (real)", |b| {
        let input = main::parse_input(&main::read_file("input"));
        b.iter(|| main::part2(black_box(&input), 4_000_000, 4_000_000))
    });
}

criterion_group!(benches, bench_main);
criterion_main!(benches);