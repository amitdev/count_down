extern crate criterion;

use count_down::count_down;
use criterion::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("CountDown", |b| {
        b.iter(|| count_down::solutions(vec![1, 3, 7, 10, 25, 50], 765))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
