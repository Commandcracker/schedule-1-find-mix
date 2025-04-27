use criterion::{Criterion, black_box, criterion_group, criterion_main};
use find_mix::ingredients::Ingredient;
use find_mix::product::Product;
use find_mix::try_all_combinations_not_threaded;
use strum::IntoEnumIterator;

fn my_benchmark(c: &mut Criterion) {
    c.bench_function("try_all_combinations", |b| {
        b.iter_with_setup(
            || {
                let available_ingredients: Vec<Ingredient> = Ingredient::iter().collect();
                (Product::OgKush, available_ingredients)
            },
            |(product, available_ingredients)| {
                black_box(try_all_combinations_not_threaded(
                    black_box(product.value()),
                    black_box(4),
                    black_box(product.effect()),
                    black_box(available_ingredients),
                ))
            },
        )
    });
}

criterion_group!(benches, my_benchmark);
criterion_main!(benches);
