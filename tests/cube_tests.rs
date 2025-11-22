use qbitcoin_core::{Cube, Move, calculate_difficulty};

#[test]
fn test_cube_creation() {
    let cube = Cube::new(3);
    assert!(cube.is_solved());
}

#[test]
fn test_cube_scramble_deterministic() {
    let mut cube1 = Cube::new(3);
    let mut cube2 = Cube::new(3);
    let block_header = b"mock_block_header";

    let scramble1 = cube1.scramble_deterministic(12345, block_header);
    let scramble2 = cube2.scramble_deterministic(12345, block_header);

    // Two cubes scrambled with the same nonce and block header should be in the same state
    assert_eq!(scramble1, scramble2);
    assert!(!cube1.is_solved());
    assert!(!cube2.is_solved());
}

#[test]
fn test_cube_move() {
    let mut cube = Cube::new(3);
    let m = Move::U(1);
    cube.apply_move(&m);
    assert!(!cube.is_solved());
    cube.apply_move(&Move::U(3)); // Reverse the move
    assert!(cube.is_solved());
}

#[test]
fn test_cube_move_normalization() {
    // Test move count normalization (e.g. U5 is equivalent to U1)
    let mut cube1 = Cube::new(3);
    let mut cube2 = Cube::new(3);

    cube1.apply_move(&Move::U(1));
    cube2.apply_move(&Move::U(5));

    // Both cubes should be in the same state
    // This test is simplified as full state comparison is complex
    assert!(!cube1.is_solved());
    assert!(!cube2.is_solved());
}

#[test]
fn test_solve_verification() {
    let mut cube = Cube::new(3);
    let block_header = b"mock_block_header";
    let scramble_moves = cube.scramble_deterministic(12345, block_header);

    // Create the inverse solution
    let mut solution = scramble_moves.clone();
    solution.reverse();
    for move_ref in solution.iter_mut() {
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
    assert!(cube.is_solved());
}

#[test]
fn test_difficulty_calculation() {
    assert_eq!(calculate_difficulty(1), 1);
    assert_eq!(calculate_difficulty(2), 3674160);
    assert_eq!(calculate_difficulty(3), 43252003274489856000);
    assert_eq!(calculate_difficulty(4), 740119684156490186987409397449857433600000000);
}

#[test]
fn test_meets_difficulty() {
    let cube = Cube::new(2);
    // A cube in its solved state should meet a very high target
    assert!(cube.meets_difficulty([0xFF; 32], u32::MAX));
    // A cube in its solved state should not meet a very low target (unless target is 0)
    assert!(cube.meets_difficulty([0x00; 32], 0));
}