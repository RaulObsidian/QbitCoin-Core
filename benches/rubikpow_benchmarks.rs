use criterion::{criterion_group, criterion_main, Criterion};
use qbitcoin_core::{Cube, Move, calculate_difficulty};

fn bench_rubikpow(c: &mut Criterion) {
    let mut group = c.benchmark_group("RubikPoW");

    for size in [3, 4, 5, 6, 7, 8, 9].iter() {
        let difficulty = calculate_difficulty(*size);
        group.bench_function(format!("solve_{}x{}x{} (difficulty: {})", size, size, size, difficulty), |b| {
            b.iter(|| {
                let mut cube = Cube::new(*size);
                let scramble_moves = cube.scramble(12345); // Fixed nonce for consistency
                // Note: This is a simplified benchmark. A full implementation would require
                // an actual solving algorithm.
                assert_eq!(cube.solve_distance(), 1);
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_rubikpow);
criterion_main!(benches);