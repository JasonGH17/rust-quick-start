use criterion::{criterion_group, criterion_main, Criterion};

use lib_logs::initialize;

fn benchmark(c: &mut Criterion) {
    initialize(
        log::LevelFilter::Trace,
        lib_logs::Output::File("./benches.log".into()),
    );

    c.bench_function("Logger init & write", |b| {
        b.iter(|| {
            log::info!("Info test");
            log::warn!("Warn test");
            log::error!("Error test");
        })
    });

    _ = std::fs::remove_file("./benches.log");
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
