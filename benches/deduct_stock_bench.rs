use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rts_assignment::{deduct_stock, generate_stock_vector};

pub fn criterion_benchmark(c: &mut Criterion) {
    // Prepare the shared stock vector
    let stock_vector = generate_stock_vector();

    // Benchmark the `add_stock` function
    c.bench_function("deduct_stock", |b| {
        b.iter(|| {
            deduct_stock(
                black_box(&stock_vector),
                black_box("Z"),
                black_box(20), 
            )
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
