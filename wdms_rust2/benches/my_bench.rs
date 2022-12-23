use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wdms_rust2::version::SemVer;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn bench_parse(c: &mut Criterion) {
    c.bench_function(
        "bllabl",
        |b| b.iter(|| "1.0.5".parse::<SemVer>()));
}


fn bench_parse2(c: &mut Criterion) {
    c.bench_function(
        "bllabl2",
        |b| b.iter(|| "1.0.a".parse::<SemVer>()));
}

criterion_group!(benches, bench_parse, bench_parse2);
criterion_main!(benches);
