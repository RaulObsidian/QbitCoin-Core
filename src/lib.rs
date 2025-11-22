use std::collections::HashMap;
use std::fmt;

use rand::Rng;
use sha3::{Digest, Sha3_256};

#[derive(Debug, Clone)]
pub struct Cube {
    size: usize,
    faces: HashMap<Face, Vec<Vec<Color>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Face {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::White => "W",
                Color::Yellow => "Y",
                Color::Red => "R",
                Color::Orange => "O",
                Color::Blue => "B",
                Color::Green => "G",
            }
        )
    }
}

impl Cube {
    pub fn new(size: usize) -> Self {
        let mut faces = HashMap::new();
        for &face in &[Face::Up, Face::Down, Face::Left, Face::Right, Face::Front, Face::Back] {
            let mut face_data = Vec::with_capacity(size);
            for _ in 0..size {
                face_data.push(vec![Color::default_for_face(face); size]);
            }
            faces.insert(face, face_data);
        }
        Cube { size, faces }
    }

    pub fn scramble(&mut self, moves: usize) -> Vec<Move> {
        let mut rng = rand::thread_rng();
        let mut scramble_moves = Vec::new();
        for _ in 0..moves {
            let random_move = Move::random(&mut rng);
            self.apply_move(&random_move);
            scramble_moves.push(random_move);
        }
        scramble_moves
    }

    pub fn apply_move(&mut self, m: &Move) {
        match m {
            Move::U(count) => {
                for _ in 0..count {
                    self.rotate_face_cw(Face::Up);
                    self.rotate_up_layer();
                }
            }
            Move::D(count) => {
                for _ in 0..count {
                    self.rotate_face_cw(Face::Down);
                    self.rotate_down_layer();
                }
            }
            Move::L(count) => {
                for _ in 0..count {
                    self.rotate_face_cw(Face::Left);
                    self.rotate_left_layer();
                }
            }
            Move::R(count) => {
                for _ in 0..count {
                    self.rotate_face_cw(Face::Right);
                    self.rotate_right_layer();
                }
            }
            Move::F(count) => {
                for _ in 0..count {
                    self.rotate_face_cw(Face::Front);
                    self.rotate_front_layer();
                }
            }
            Move::B(count) => {
                for _ in 0..count {
                    self.rotate_face_cw(Face::Back);
                    self.rotate_back_layer();
                }
            }
        }
    }

    fn rotate_face_cw(&mut self, face: Face) {
        let mut face_data = self.faces.get_mut(&face).unwrap();
        let n = self.size;
        for i in 0..n / 2 {
            for j in i..n - i - 1 {
                let temp = face_data[i][j];
                face_data[i][j] = face_data[n - j - 1][i];
                face_data[n - j - 1][i] = face_data[n - i - 1][n - j - 1];
                face_data[n - i - 1][n - j - 1] = face_data[j][n - i - 1];
                face_data[j][n - i - 1] = temp;
            }
        }
    }

    fn rotate_up_layer(&mut self) {
        let n = self.size;
        let left_col: Vec<Color> = (0..n).map(|i| *self.faces[&Face::Front][0][i]).collect();
        for i in 0..n {
            *self.faces.get_mut(&Face::Front).unwrap()[0][i] = self.faces[&Face::Right][0][i];
            *self.faces.get_mut(&Face::Right).unwrap()[0][i] = self.faces[&Face::Back][0][i];
            *self.faces.get_mut(&Face::Back).unwrap()[0][i] = self.faces[&Face::Left][0][i];
            *self.faces.get_mut(&Face::Left).unwrap()[0][i] = left_col[i];
        }
    }

    fn rotate_down_layer(&mut self) {
        let n = self.size;
        let left_col: Vec<Color> = (0..n).map(|i| *self.faces[&Face::Front][n - 1][i]).collect();
        for i in 0..n {
            *self.faces.get_mut(&Face::Front).unwrap()[n - 1][i] = self.faces[&Face::Left][n - 1][i];
            *self.faces.get_mut(&Face::Left).unwrap()[n - 1][i] = self.faces[&Face::Back][n - 1][i];
            *self.faces.get_mut(&Face::Back).unwrap()[n - 1][i] = self.faces[&Face::Right][n - 1][i];
            *self.faces.get_mut(&Face::Right).unwrap()[n - 1][i] = left_col[i];
        }
    }

    fn rotate_left_layer(&mut self) {
        let n = self.size;
        let up_row: Vec<Color> = (0..n).map(|i| *self.faces[&Face::Up][i][0]).collect();
        for i in 0..n {
            *self.faces.get_mut(&Face::Up).unwrap()[i][0] = self.faces[&Face::Back][n - i - 1][n - 1];
            *self.faces.get_mut(&Face::Back).unwrap()[n - i - 1][n - 1] = self.faces[&Face::Down][i][0];
            *self.faces.get_mut(&Face::Down).unwrap()[i][0] = self.faces[&Face::Front][i][0];
            *self.faces.get_mut(&Face::Front).unwrap()[i][0] = up_row[i];
        }
    }

    fn rotate_right_layer(&mut self) {
        let n = self.size;
        let up_row: Vec<Color> = (0..n).map(|i| *self.faces[&Face::Up][i][n - 1]).collect();
        for i in 0..n {
            *self.faces.get_mut(&Face::Up).unwrap()[i][n - 1] = self.faces[&Face::Front][i][n - 1];
            *self.faces.get_mut(&Face::Front).unwrap()[i][n - 1] = self.faces[&Face::Down][i][n - 1];
            *self.faces.get_mut(&Face::Down).unwrap()[i][n - 1] = self.faces[&Face::Back][n - i - 1][0];
            *self.faces.get_mut(&Face::Back).unwrap()[n - i - 1][0] = up_row[i];
        }
    }

    fn rotate_front_layer(&mut self) {
        let n = self.size;
        let up_row: Vec<Color> = self.faces[&Face::Up][n - 1].clone();
        for i in 0..n {
            self.faces.get_mut(&Face::Up).unwrap()[n - 1][i] = self.faces[&Face::Left][n - i - 1][n - 1];
            self.faces.get_mut(&Face::Left).unwrap()[n - i - 1][n - 1] = self.faces[&Face::Down][0][n - i - 1];
            self.faces.get_mut(&Face::Down).unwrap()[0][n - i - 1] = self.faces[&Face::Right][i][0];
            self.faces.get_mut(&Face::Right).unwrap()[i][0] = up_row[i];
        }
    }

    fn rotate_back_layer(&mut self) {
        let n = self.size;
        let up_row: Vec<Color> = self.faces[&Face::Up][0].clone();
        for i in 0..n {
            self.faces.get_mut(&Face::Up).unwrap()[0][i] = self.faces[&Face::Right][i][n - 1];
            self.faces.get_mut(&Face::Right).unwrap()[i][n - 1] = self.faces[&Face::Down][n - 1][n - i - 1];
            self.faces.get_mut(&Face::Down).unwrap()[n - 1][n - i - 1] = self.faces[&Face::Left][n - i - 1][0];
            self.faces.get_mut(&Face::Left).unwrap()[n - i - 1][0] = up_row[i];
        }
    }

    pub fn is_solved(&self) -> bool {
        for &face in &[Face::Up, Face::Down, Face::Left, Face::Right, Face::Front, Face::Back] {
            let face_data = &self.faces[&face];
            let center_color = face_data[self.size / 2][self.size / 2];
            for row in face_data {
                for &color in row {
                    if color != center_color {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn solve_distance(&self) -> usize {
        if self.is_solved() {
            0
        } else {
            // This is a simplified version. A full implementation would require a more complex
            // algorithm like IDA* or Korf's algorithm.
            1
        }
    }

    pub fn verify_solution(&self, moves: &[Move]) -> bool {
        let mut cube = self.clone();
        for m in moves {
            cube.apply_move(m);
        }
        cube.is_solved()
    }
}

impl Color {
    pub fn default_for_face(face: Face) -> Self {
        match face {
            Face::Up => Color::White,
            Face::Down => Color::Yellow,
            Face::Front => Color::Red,
            Face::Back => Color::Orange,
            Face::Left => Color::Blue,
            Face::Right => Color::Green,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    U(usize),
    D(usize),
    L(usize),
    R(usize),
    F(usize),
    B(usize),
}

impl Move {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        use Move::*;
        let move_type = rng.gen_range(0..6);
        let count = rng.gen_range(1..4);
        match move_type {
            0 => U(count),
            1 => D(count),
            2 => L(count),
            3 => R(count),
            4 => F(count),
            5 => B(count),
            _ => unreachable!(),
        }
    }
}

pub fn calculate_difficulty(n: usize) -> u32 {
    (n * n * n) as u32
}