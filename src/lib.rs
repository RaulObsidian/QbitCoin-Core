use std::collections::HashMap;
use std::fmt;

use rand::Rng;
use sha3::{Digest, Sha3_256};
use tiny_keccak::{Hasher, Keccak};

#[derive(Debug, Clone)]
pub struct Cube {
    size: usize,
    // For n x n x n cube, we need to track corner and edge permutations and orientations
    // This is a simplified representation. A full implementation would require
    // more complex data structures to handle all cubies.
    corners: [[usize; 3]; 8], // Corner cubie positions (8 corners)
    edges: [[usize; 2]; 12], // Edge cubie positions (12 edges)
    // Orientation states for corners and edges
    corner_orientations: [u8; 8], // 0, 1, 2 for 3 orientations
    edge_orientations: [u8; 12],  // 0, 1 for 2 orientations
    // Color faces (for visualization and solving checks)
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

        // Initialize corner and edge positions (simplified for this example)
        let corners = [
            [0, 1, 2], [0, 2, 5], [0, 5, 4], [0, 4, 1], // Top layer
            [5, 2, 3], [5, 3, 6], [5, 6, 7], [5, 7, 4], // Bottom layer
        ];
        let edges = [
            [0, 1], [0, 2], [0, 4], [0, 3], // Top layer
            [1, 2], [2, 5], [5, 4], [4, 1], // Middle layer
            [3, 6], [6, 7], [7, 4], [3, 7], // Bottom layer
        ];

        Cube {
            size,
            corners,
            edges,
            corner_orientations: [0; 8],
            edge_orientations: [0; 12],
            faces,
        }
    }

    pub fn scramble_deterministic(&mut self, nonce: u64, block_header: &[u8]) -> Vec<Move> {
        // Create a deterministic scramble from the nonce and block header
        let mut hasher = Sha3_256::new();
        hasher.update(nonce.to_le_bytes());
        hasher.update(block_header);
        let hash = hasher.finalize();

        // Use the hash to seed a random number generator for deterministic scrambling
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&hash);
        let mut rng = rand::rngs::StdRng::from_seed(seed);

        let num_moves = rng.gen_range(20..=30); // Standard scramble length

        let mut scramble_moves = Vec::new();
        let mut last_face: Option<Face> = None;

        for _ in 0..num_moves {
            let mut random_face;
            loop {
                random_face = [
                    Face::Up, Face::Down, Face::Left,
                    Face::Right, Face::Front, Face::Back
                ][rng.gen_range(0..6)];

                // Avoid redundant moves (e.g. R R')
                if last_face != Some(random_face) {
                    break;
                }
            }

            let count = rng.gen_range(1..4); // 1, 2, or 3 rotations
            let random_move = Move::from_face_and_count(random_face, count);

            self.apply_move(&random_move);
            scramble_moves.push(random_move);

            last_face = Some(random_face);
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

        // Update corner and edge orientations based on the face rotation
        self.update_orientations_for_face_rotation(face);
    }

    fn update_orientations_for_face_rotation(&mut self, face: Face) {
        // Update orientations based on which face was rotated
        // This is a simplified representation; a full implementation would
        // correctly update all affected cubie orientations.
        match face {
            Face::Up => {
                // Update orientations for U face rotation
                self.corner_orientations[0] = (self.corner_orientations[0] + 1) % 3;
                self.corner_orientations[1] = (self.corner_orientations[1] + 1) % 3;
                self.corner_orientations[2] = (self.corner_orientations[2] + 1) % 3;
                self.corner_orientations[3] = (self.corner_orientations[3] + 1) % 3;
            },
            Face::Down => {
                // Update orientations for D face rotation
                self.corner_orientations[4] = (self.corner_orientations[4] + 1) % 3;
                self.corner_orientations[5] = (self.corner_orientations[5] + 1) % 3;
                self.corner_orientations[6] = (self.corner_orientations[6] + 1) % 3;
                self.corner_orientations[7] = (self.corner_orientations[7] + 1) % 3;
            },
            Face::Front => {
                // Update orientations for F face rotation
                self.edge_orientations[4] = (self.edge_orientations[4] + 1) % 2;
                self.edge_orientations[5] = (self.edge_orientations[5] + 1) % 2;
                self.edge_orientations[6] = (self.edge_orientations[6] + 1) % 2;
                self.edge_orientations[7] = (self.edge_orientations[7] + 1) % 2;
            },
            Face::Back => {
                // Update orientations for B face rotation
                self.edge_orientations[8] = (self.edge_orientations[8] + 1) % 2;
                self.edge_orientations[9] = (self.edge_orientations[9] + 1) % 2;
                self.edge_orientations[10] = (self.edge_orientations[10] + 1) % 2;
                self.edge_orientations[11] = (self.edge_orientations[11] + 1) % 2;
            },
            Face::Left => {
                // Update orientations for L face rotation
                // No edge orientation changes for L face rotation
            },
            Face::Right => {
                // Update orientations for R face rotation
                // No edge orientation changes for R face rotation
            },
        }
    }

    fn rotate_up_layer(&mut self) {
        // Rotate the up layer (affects corners and edges)
        // This is a simplified representation
        let temp_corners = self.corners[0];
        self.corners[0] = self.corners[3];
        self.corners[3] = self.corners[2];
        self.corners[2] = self.corners[1];
        self.corners[1] = temp_corners;

        let temp_edges = self.edges[0];
        self.edges[0] = self.edges[3];
        self.edges[3] = self.edges[2];
        self.edges[2] = self.edges[1];
        self.edges[1] = temp_edges;
    }

    fn rotate_down_layer(&mut self) {
        // Rotate the down layer (affects corners and edges)
        // This is a simplified representation
        let temp_corners = self.corners[4];
        self.corners[4] = self.corners[5];
        self.corners[5] = self.corners[6];
        self.corners[6] = self.corners[7];
        self.corners[7] = temp_corners;

        let temp_edges = self.edges[8];
        self.edges[8] = self.edges[9];
        self.edges[9] = self.edges[10];
        self.edges[10] = self.edges[11];
        self.edges[11] = temp_edges;
    }

    fn rotate_left_layer(&mut self) {
        // Rotate the left layer (affects corners and edges)
        // This is a simplified representation
        let temp_corners = self.corners[0];
        self.corners[0] = self.corners[4];
        self.corners[4] = self.corners[7];
        self.corners[7] = self.corners[3];
        self.corners[3] = temp_corners;

        let temp_edges = self.edges[2];
        self.edges[2] = self.edges[8];
        self.edges[8] = self.edges[10];
        self.edges[10] = self.edges[4];
        self.edges[4] = temp_edges;
    }

    fn rotate_right_layer(&mut self) {
        // Rotate the right layer (affects corners and edges)
        // This is a simplified representation
        let temp_corners = self.corners[1];
        self.corners[1] = self.corners[2];
        self.corners[2] = self.corners[6];
        self.corners[6] = self.corners[5];
        self.corners[5] = temp_corners;

        let temp_edges = self.edges[1];
        self.edges[1] = self.edges[5];
        self.edges[5] = self.edges[9];
        self.edges[9] = self.edges[7];
        self.edges[7] = temp_edges;
    }

    fn rotate_front_layer(&mut self) {
        // Rotate the front layer (affects corners and edges)
        // This is a simplified representation
        let temp_corners = self.corners[0];
        self.corners[0] = self.corners[1];
        self.corners[1] = self.corners[2];
        self.corners[2] = self.corners[3];
        self.corners[3] = temp_corners;

        let temp_edges = self.edges[0];
        self.edges[0] = self.edges[1];
        self.edges[1] = self.edges[2];
        self.edges[2] = self.edges[3];
        self.edges[3] = temp_edges;
    }

    fn rotate_back_layer(&mut self) {
        // Rotate the back layer (affects corners and edges)
        // This is a simplified representation
        let temp_corners = self.corners[4];
        self.corners[4] = self.corners[7];
        self.corners[7] = self.corners[6];
        self.corners[6] = self.corners[5];
        self.corners[5] = temp_corners;

        let temp_edges = self.edges[8];
        self.edges[8] = self.edges[11];
        self.edges[11] = self.edges[10];
        self.edges[10] = self.edges[9];
        self.edges[9] = temp_edges;
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

        // Check if corners and edges are in their original positions
        for i in 0..8 {
            if self.corners[i] != [i, i+1, i+2] { // Simplified check
                return false;
            }
        }

        for i in 0..12 {
            if self.edges[i] != [i, i+1] { // Simplified check
                return false;
            }
        }

        // Check orientations
        for &orientation in &self.corner_orientations {
            if orientation != 0 {
                return false;
            }
        }

        for &orientation in &self.edge_orientations {
            if orientation != 0 {
                return false;
            }
        }

        true
    }

    pub fn verify_solution(&self, moves: &[Move]) -> bool {
        let mut cube = self.clone();
        for m in moves {
            cube.apply_move(m);
        }
        cube.is_solved()
    }

    pub fn meets_difficulty(&self, hash: [u8; 32], target: u32) -> bool {
        // Convert the hash to a number and compare with the target
        // This is a simplified representation; a full implementation would
        // correctly interpret the hash and target values.

        let mut hasher = Keccak::v256();
        let mut result = [0u8; 32];

        // Create a string representation of the cube state
        let cube_state = format!("{:?}", self.faces);

        hasher.update(cube_state.as_bytes());
        hasher.finalize(&mut result);

        // Convert the first 4 bytes of the hash to a u32 for comparison
        let hash_value = u32::from_le_bytes([result[0], result[1], result[2], result[3]]);

        hash_value <= target
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
    U(usize),   // Up face clockwise
    D(usize),   // Down face clockwise
    L(usize),   // Left face clockwise
    R(usize),   // Right face clockwise
    F(usize),   // Front face clockwise
    B(usize),   // Back face clockwise
}

impl Move {
    pub fn from_face_and_count(face: Face, count: usize) -> Self {
        match face {
            Face::Up => Move::U(count % 4), // Normalize count to 0, 1, 2, or 3
            Face::Down => Move::D(count % 4),
            Face::Left => Move::L(count % 4),
            Face::Right => Move::R(count % 4),
            Face::Front => Move::F(count % 4),
            Face::Back => Move::B(count % 4),
        }
    }
}

pub fn calculate_difficulty(n: usize) -> u32 {
    // Use the number of possible states as a measure of difficulty
    // For a 3x3x3: ~4.32e19 states
    // For higher n, the number of states grows factorially
    match n {
        1 => 1,
        2 => 3674160, // 2x2x2 has 3,674,160 states
        3 => 43252003274489856000, // 3x3x3 has ~4.3e19 states
        4 => 740119684156490186987409397449857433600000000, // 4x4x4 has ~7.4e45 states
        _ => {
            // For n > 4, we use a simplified approximation
            // The actual number of states for n>4 is much more complex to calculate
            (n * n * n * 24) as u32 // Simplified approximation
        }
    }
}