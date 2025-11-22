use criterion::{criterion_group, criterion_main, Criterion};
use qbitcoin_core::{Cube, Move, calculate_difficulty};

fn bench_rubikpow(c: &mut Criterion) {
    let mut group = c.benchmark_group("RubikPoW");

    // Benchmark for different cube sizes
    for size in [3, 4, 5].iter() {
        let difficulty = calculate_difficulty(*size);
        group.bench_function(format!("verify_{}x{}x{} (difficulty: {})", size, size, size, difficulty), |b| {
            b.iter(|| {
                let mut cube = Cube::new(*size);
                let block_header = b"mock_block_header";
                let scramble_moves = cube.scramble_deterministic(12345, block_header);
                // Verify the scramble_moves solve the cube (reversing the scramble)
                let mut solution = scramble_moves.clone();
                solution.reverse();
                for move_ref in solution.iter_mut() {
                    // Invert each move (U -> U', U' -> U, U2 -> U2)
                    match move_ref {
                        Move::U(count) => *move_ref = Move::U((4 - count) % 4),
                        Move::D(count) => *move_ref = Move::D((4 - count) % 4),
                        Move::L(count) => *move_ref = Move::L((4 - count) % 4),
                        Move::R(count) => *move_ref = Move::R((4 - count) % 4),
                        Move::F(count) => *move_ref = Move::F((4 - count) % 4),
                        Move::B(count) => *move_ref = Move::B((4 - count) % 4),
                    }
                }
                assert!(cube.verify_solution(&solution));
            })
        });
    }

    group.finish();

    // Add notes about quantum complexity
    println!("NOTE: Grover's algorithm would require 2^89 - 2^193 quantum operations for 3x3x3 - 5x5x5 cubes.");
}

criterion_group!(benches, bench_rubikpow);
criterion_main!(benches);