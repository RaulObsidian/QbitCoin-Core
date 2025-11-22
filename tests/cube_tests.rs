use qbitcoin_core::{Cube, Move};

#[test]
fn test_cube_creation() {
    let cube = Cube::new(3);
    assert!(cube.is_solved());
}

#[test]
fn test_cube_scramble() {
    let mut cube = Cube::new(3);
    let scramble_moves = cube.scramble(20);
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
    cube.scramble(1);
    assert_eq!(cube.solve_distance(), 1);
}