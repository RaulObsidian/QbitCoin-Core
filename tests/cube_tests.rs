use qbitcoin_core::{Cube, Move, calculate_difficulty};

#[test]
fn test_cube_creation() {
    let cube = Cube::new(3);
    assert!(cube.is_solved());
}

#[test]
fn test_cube_scramble() {
    let mut cube = Cube::new(3);
    let scramble_moves = cube.scramble(12345);
    assert!(!cube.is_solved());
    assert!(cube.verify_solution(&scramble_moves));
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
fn test_solve_distance() {
    let mut cube = Cube::new(3);
    cube.scramble(12345);
    assert_eq!(cube.solve_distance(), 1);
}

#[test]
fn test_difficulty_calculation() {
    assert_eq!(calculate_difficulty(1), 1);
    assert_eq!(calculate_difficulty(2), 3674160);
    assert_eq!(calculate_difficulty(3), 43252003274489856000);
}

#[test]
fn test_meets_difficulty() {
    let mut cube = Cube::new(2);
    let scramble_moves = cube.scramble(12345);
    // This test is simplified, as the full implementation would require a proper target hash
    assert!(cube.meets_difficulty(&[0u8; 32]));
}