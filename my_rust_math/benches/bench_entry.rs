use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/prime_num.rs"]
mod prime_num;

fn bench_signal_repo_static(c: &mut Criterion) {
    c.bench_function("get_prime_cnt_rust1/1024", |b| {
        b.iter(|| {
            black_box(prime_num::get_prime_cnt_rust1(1024));
        })
    });
}

criterion_group!(benches, bench_signal_repo_static,);
criterion_main!(benches);
