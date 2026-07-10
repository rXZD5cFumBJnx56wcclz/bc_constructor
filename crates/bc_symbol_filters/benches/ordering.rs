mod prelude;
use std::hint::black_box;

use prelude::*;

use bc_symbol_filters::ordering::*;

fn symbol_filter_1(c: &mut Criterion) {
    let symbol_filter = ORDERING::new(1., "less".to_string());
    c.bench_function("symbol_filter_1", |b| {
        b.iter(|| symbol_filter.symbol_filter(black_box(&[]), black_box(&[0.9])))
    });
}

criterion_group!(benches, symbol_filter_1);
criterion_main!(benches);
