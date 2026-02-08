use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use tmfroc::conway;
use tmfroc::types::cell_configuration::CellConfiguration;

fn bench_simulation(c: &mut Criterion) {
    let seed_cells = CellConfiguration::random_configuration(42, 50, 50, 0.4);
    let input = CellConfiguration::with_seed_configuration(seed_cells);

    c.bench_function("conway", |b| {
        b.iter(|| {
            conway::simulation::simulation(black_box(&input));
        })
    });

    c.bench_function("thanatos", |b| {
        b.iter(|| {
            tmfroc::thanatos::tmfroc::run(black_box(&input));
        })
    });
}

criterion_group!(benches, bench_simulation);
criterion_main!(benches);
