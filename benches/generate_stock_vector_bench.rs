use criterion::{criterion_group,criterion_main,Criterion};
use rts_assignment::generate_stock_vector;

pub fn criterion_benchmark(c:&mut Criterion){
    c.bench_function("generate_stock_vector",|b | b.iter(|| generate_stock_vector()));
}

criterion_group!(benches,criterion_benchmark);
criterion_main!(benches);