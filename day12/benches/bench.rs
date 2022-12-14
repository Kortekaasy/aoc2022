#![allow(dead_code)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/main.rs"]
mod main;

fn bench_main(c: &mut Criterion) {
    // c.bench_function("parse input (sample)", |b| {
    //     let file = main::read_file("sample");
    //     b.iter(|| main::parse_input::<40>(black_box(&file)))
    // });

    c.bench_function("part 1 (sample)", |b| {
        let input = main::parse_input::<40>(&main::read_file("sample"));
        b.iter(|| main::part1(black_box(&input)))
    });

    c.bench_function("part 2 (sample)", |b| {
        let input = main::parse_input::<40>(&main::read_file("sample"));
        b.iter(|| main::part2(black_box(&input)))
    });

    // c.bench_function("parse input (real)", |b| {
    //     let file = main::read_file("input");
    //     b.iter(|| main::parse_input::<6847>(black_box(&file)))
    // });

    c.bench_function("part 1 (real)", |b| {
        let input = main::parse_input::<6847>(&main::read_file("input"));
        b.iter(|| main::part1(black_box(&input)))
    });
    
    c.bench_function("part 2 (real)", |b| {
        let input = main::parse_input::<6847>(&main::read_file("input"));
        b.iter(|| main::part2(black_box(&input)))
    });
}

criterion_group!(benches, bench_main);
criterion_main!(benches);