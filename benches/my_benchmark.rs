use criterion::{Criterion, black_box, criterion_group, criterion_main};
use find_mix::product::Product;
use find_mix::try_all_combinations;

fn my_benchmark(c: &mut Criterion) {
    c.bench_function("try_all_combinations", |b| {
        b.iter_with_setup(
            || Product::OgKush,
            |product| {
                black_box(try_all_combinations(
                    black_box(product.value()),
                    black_box(4),
                    black_box(product.effect()),
                ))
            },
        )
    });
}

criterion_group!(benches, my_benchmark);
criterion_main!(benches);
