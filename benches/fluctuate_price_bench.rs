use criterion::{criterion_group,criterion_main,Criterion};
use rts_assignment::{fluctuate_price,generate_stock_vector};

pub fn criterion_benchmark(c:&mut Criterion){
    let stock_vector = generate_stock_vector();

    c.bench_function("fluctuate_price", |b| {
        b.iter(|| {
            // Lock the mutex to access the stock vector
            let mut stock_vector = stock_vector.lock().unwrap();
            for stock in stock_vector.iter_mut() {
                fluctuate_price(stock); // Pass each stock one by one
            }
        })
    });
}

criterion_group!(benches,criterion_benchmark);
criterion_main!(benches);