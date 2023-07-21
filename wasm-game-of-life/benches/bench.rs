extern crate wasm_game_of_life;
use criterion::{criterion_group, criterion_main, Criterion};

fn universe_tick_benchmark(c: &mut Criterion) {
    let mut universe = wasm_game_of_life::Universe::new(64, 64);
    c.bench_function("tick", |b| b.iter(|| universe.tick()));
}

criterion_group!(benches, universe_tick_benchmark);
criterion_main!(benches);
