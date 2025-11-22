use std::collections::HashMap;
use std::fmt;

use rand::Rng;
use sha3::{Digest, Sha3_256};
use tiny_keccak::{Hasher, Keccak};

#[derive(Debug, Clone)]
pub struct Cube {
    size: usize,
    faces: HashMap<Face, Vec<Vec<Color>>>,
    // Orientation tracking for each cubie
    orientations: Vec<Vec<Vec<CubieOrientation>>>,
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

#[derive(Debug, Clone, Copy, Default)]
struct CubieOrientation {
    /// Orientation value for each of the 3 axes (x, y, z)
    /// Values: 0 (default), 1, 2 (rotated)
    orientation: [u8; 3],
}

impl CubieOrientation {
    fn apply_rotation(&mut self, axis: usize) {
        // Apply rotation to the orientation state
        // This is a simplified model; a full implementation would track
        // all 3D orientations and parity changes.
        self.orientation[axis] = (self.orientation[axis] + 1) % 4;
    }

    fn is_aligned(&self) -> bool {
        self.orientation == [0, 0, 0]
    }
}

impl Cube {
    pub fn new(size: usize) -> Self {
        let mut faces = HashMap::new();
        let mut orientations = Vec::new();

        for &face in &[Face::Up, Face::Down, Face::Left, Face::Right, Face::Front, Face::Back] {
            let mut face_data = Vec::with_capacity(size);
            for _ in 0..size {
                face_data.push(vec![Color::default_for_face(face); size]);
            }
            faces.insert(face, face_data);
        }

        for _ in 0..size {
            let mut y_layer = Vec::new();
            for _ in 0..size {
                let mut x_row = Vec::new();
                for _ in 0..size {
                    x_row.push(CubieOrientation::default());
                }
                y_layer.push(x_row);
            }
            orientations.push(y_layer);
        }

        Cube { size, faces, orientations }
    }

    pub fn scramble(&mut self, nonce: u64) -> Vec<Move> {
        // Create a deterministic scramble from the nonce
        let mut rng = rand::rngs::StdRng::seed_from_u64(nonce);
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
            Move::U(count) | Move::Uw(count) | Move::X(count) => {
                for _ in 0..count {
                    self.rotate_face_cw(Face::Up);
                    self.rotate_up_layer();
                    if matches!(m, Move::Uw(_)) {
                        self.rotate_middle_layers(1, self.size - 2, Axis::Y);
                    } else if matches!(m, Move::X(_)) {
                        self.rotate_entire_cube_x();
                    }
                }
            }
            Move::D(count) | Move::Dw(count) | Move::X(count) => {
                for _ in 0..count {
                    self.rotate_face_cw(Face::Down);
                    self.rotate_down_layer();
                    if matches!(m, Move::Dw(_)) {
                        self.rotate_middle_layers(1, self.size - 2, Axis::Y);
                    } else if matches!(m, Move::X(_)) {
                        self.rotate_entire_cube_x();
                    }
                }
            }
            Move::L(count) | Move::Lw(count) | Move::Y(count) => {
                for _ in 0..count {
                    self.rotate_face_cw(Face::Left);
                    self.rotate_left_layer();
                    if matches!(m, Move::Lw(_)) {
                        self.rotate_middle_layers(1, self.size - 2, Axis::X);
                    } else if matches!(m, Move::Y(_)) {
                        self.rotate_entire_cube_y();
                    }
                }
            }
            Move::R(count) | Move::Rw(count) | Move::Y(count) => {
                for _ in 0..count {
                    self.rotate_face_cw(Face::Right);
                    self.rotate_right_layer();
                    if matches!(m, Move::Rw(_)) {
                        self.rotate_middle_layers(1, self.size - 2, Axis::X);
                    } else if matches!(m, Move::Y(_)) {
                        self.rotate_entire_cube_y();
                    }
                }
            }
            Move::F(count) | Move::Fw(count) | Move::Z(count) => {
                for _ in 0..count {
                    self.rotate_face_cw(Face::Front);
                    self.rotate_front_layer();
                    if matches!(m, Move::Fw(_)) {
                        self.rotate_middle_layers(1, self.size - 2, Axis::Z);
                    } else if matches!(m, Move::Z(_)) {
                        self.rotate_entire_cube_z();
                    }
                }
            }
            Move::B(count) | Move::Bw(count) | Move::Z(count) => {
                for _ in 0..count {
                    self.rotate_face_cw(Face::Back);
                    self.rotate_back_layer();
                    if matches!(m, Move::Bw(_)) {
                        self.rotate_middle_layers(1, self.size - 2, Axis::Z);
                    } else if matches!(m, Move::Z(_)) {
                        self.rotate_entire_cube_z();
                    }
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

        // Update orientations for the face
        self.rotate_orientation_face_cw(face);
    }

    fn rotate_orientation_face_cw(&mut self, face: Face) {
        let (start_x, start_y, size) = match face {
            Face::Up => (0, 0, self.size),
            Face::Down => (0, self.size - 1, self.size),
            Face::Left => (0, 0, self.size),
            Face::Right => (self.size - 1, 0, self.size),
            Face::Front => (0, 0, self.size),
            Face::Back => (self.size - 1, 0, self.size),
        };

        // Rotate orientations in the same pattern as colors
        for i in start_y..start_y + size {
            for j in start_x..start_x + size {
                if i < self.orientations.len() && j < self.orientations[i].len() {
                    // Apply rotation logic to orientation
                    // This is a simplified representation
                }
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

        // Update orientations for the affected cubies
        self.rotate_orientations_up_layer();
    }

    fn rotate_orientations_up_layer(&mut self) {
        // Update orientations when rotating the up layer
        let n = self.size;
        let front_row = self.orientations[0][0].clone(); // Top row of front face
        for i in 0..n {
            self.orientations[0][i][0] = self.orientations[0][n - 1 - i][n - 1]; // From right to front
            self.orientations[0][n - 1 - i][n - 1] = self.orientations[0][n - 1][n - 1 - i]; // From back to right
            self.orientations[0][n - 1][n - 1 - i] = self.orientations[0][i][0]; // From left to back
            self.orientations[0][i][0] = front_row[i]; // From front to left
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

        // Update orientations for the affected cubies
        self.rotate_orientations_down_layer();
    }

    fn rotate_orientations_down_layer(&mut self) {
        // Update orientations when rotating the down layer
        let n = self.size;
        let front_row = self.orientations[n - 1][0].clone(); // Bottom row of front face
        for i in 0..n {
            self.orientations[n - 1][i][0] = self.orientations[n - 1][i][n - 1]; // From front to left
            self.orientations[n - 1][i][n - 1] = self.orientations[n - 1][n - 1 - i][n - 1]; // From left to back
            self.orientations[n - 1][n - 1 - i][n - 1] = self.orientations[n - 1][n - 1][n - 1 - i]; // From back to right
            self.orientations[n - 1][n - 1][n - 1 - i] = front_row[n - 1 - i]; // From right to front
        }
    }

    fn rotate_left_layer(&mut self) {
        let n = self.size;
        let up_col: Vec<Color> = (0..n).map(|i| *self.faces[&Face::Up][i][0]).collect();
        for i in 0..n {
            *self.faces.get_mut(&Face::Up).unwrap()[i][0] = self.faces[&Face::Back][n - i - 1][n - 1];
            *self.faces.get_mut(&Face::Back).unwrap()[n - i - 1][n - 1] = self.faces[&Face::Down][i][0];
            *self.faces.get_mut(&Face::Down).unwrap()[i][0] = self.faces[&Face::Front][i][0];
            *self.faces.get_mut(&Face::Front).unwrap()[i][0] = up_col[i];
        }

        // Update orientations for the affected cubies
        self.rotate_orientations_left_layer();
    }

    fn rotate_orientations_left_layer(&mut self) {
        // Update orientations when rotating the left layer
        let n = self.size;
        let up_col = (0..n).map(|i| self.orientations[i][0][0]).collect::<Vec<_>>();
        for i in 0..n {
            self.orientations[i][0][0] = self.orientations[n - 1 - i][n - 1][n - 1]; // From back to up
            self.orientations[n - 1 - i][n - 1][n - 1] = self.orientations[i][n - 1][0]; // From down to back
            self.orientations[i][n - 1][0] = self.orientations[n - 1 - i][0][0]; // From front to down
            self.orientations[n - 1 - i][0][0] = up_col[i]; // From up to front
        }
    }

    fn rotate_right_layer(&mut self) {
        let n = self.size;
        let up_col: Vec<Color> = (0..n).map(|i| *self.faces[&Face::Up][i][n - 1]).collect();
        for i in 0..n {
            *self.faces.get_mut(&Face::Up).unwrap()[i][n - 1] = self.faces[&Face::Front][i][n - 1];
            *self.faces.get_mut(&Face::Front).unwrap()[i][n - 1] = self.faces[&Face::Down][i][n - 1];
            *self.faces.get_mut(&Face::Down).unwrap()[i][n - 1] = self.faces[&Face::Back][n - i - 1][0];
            *self.faces.get_mut(&Face::Back).unwrap()[n - i - 1][0] = up_col[i];
        }

        // Update orientations for the affected cubies
        self.rotate_orientations_right_layer();
    }

    fn rotate_orientations_right_layer(&mut self) {
        // Update orientations when rotating the right layer
        let n = self.size;
        let up_col = (0..n).map(|i| self.orientations[i][0][n - 1]).collect::<Vec<_>>();
        for i in 0..n {
            self.orientations[i][0][n - 1] = self.orientations[i][0][0]; // From up to front
            self.orientations[i][0][0] = self.orientations[i][n - 1][0]; // From front to down
            self.orientations[i][n - 1][0] = self.orientations[n - 1 - i][n - 1][n - 1]; // From down to back
            self.orientations[n - 1 - i][n - 1][n - 1] = up_col[i]; // From back to up
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

        // Update orientations for the affected cubies
        self.rotate_orientations_front_layer();
    }

    fn rotate_orientations_front_layer(&mut self) {
        // Update orientations when rotating the front layer
        let n = self.size;
        let up_row = self.orientations[n - 1][0].clone();
        for i in 0..n {
            self.orientations[n - 1][i][0] = self.orientations[n - 1 - i][n - 1][n - 1]; // From left to up
            self.orientations[n - 1 - i][n - 1][n - 1] = self.orientations[0][n - 1 - i][n - 1]; // From down to left
            self.orientations[0][n - 1 - i][n - 1] = self.orientations[i][0][n - 1]; // From right to down
            self.orientations[i][0][n - 1] = up_row[i]; // From up to right
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

        // Update orientations for the affected cubies
        self.rotate_orientations_back_layer();
    }

    fn rotate_orientations_back_layer(&mut self) {
        // Update orientations when rotating the back layer
        let n = self.size;
        let up_row = self.orientations[0][0].clone();
        for i in 0..n {
            self.orientations[0][i][0] = self.orientations[i][n - 1][n - 1]; // From up to right
            self.orientations[i][n - 1][n - 1] = self.orientations[n - 1][n - 1 - i][n - 1]; // From right to down
            self.orientations[n - 1][n - 1 - i][n - 1] = self.orientations[n - 1 - i][0][n - 1]; // From down to left
            self.orientations[n - 1 - i][0][n - 1] = up_row[i]; // From left to up
        }
    }

    fn rotate_middle_layers(&mut self, start: usize, end: usize, axis: Axis) {
        // Rotate middle layers along the specified axis
        match axis {
            Axis::X => {
                for layer in start..=end {
                    self.rotate_layer_x(layer);
                }
            }
            Axis::Y => {
                for layer in start..=end {
                    self.rotate_layer_y(layer);
                }
            }
            Axis::Z => {
                for layer in start..=end {
                    self.rotate_layer_z(layer);
                }
            }
        }
    }

    fn rotate_layer_x(&mut self, layer: usize) {
        // Rotate a layer along the X axis (like R, M, L moves combined)
        // Implementation would depend on the specific cube size and layer
        // This is a simplified representation
    }

    fn rotate_layer_y(&mut self, layer: usize) {
        // Rotate a layer along the Y axis (like U, E, D moves combined)
        // Implementation would depend on the specific cube size and layer
        // This is a simplified representation
    }

    fn rotate_layer_z(&mut self, layer: usize) {
        // Rotate a layer along the Z axis (like F, S, B moves combined)
        // Implementation would depend on the specific cube size and layer
        // This is a simplified representation
    }

    fn rotate_entire_cube_x(&mut self) {
        // Rotate the entire cube around the X axis
        // This affects all layers
        self.rotate_face_cw(Face::Up);
        self.rotate_face_ccw(Face::Down);
        // Additional rotations for other faces
    }

    fn rotate_entire_cube_y(&mut self) {
        // Rotate the entire cube around the Y axis
        // This affects all layers
        self.rotate_face_cw(Face::Front);
        self.rotate_face_ccw(Face::Back);
        // Additional rotations for other faces
    }

    fn rotate_entire_cube_z(&mut self) {
        // Rotate the entire cube around the Z axis
        // This affects all layers
        self.rotate_face_cw(Face::Right);
        self.rotate_face_ccw(Face::Left);
        // Additional rotations for other faces
    }

    fn rotate_face_ccw(&mut self, face: Face) {
        // Counter-clockwise rotation is 3 clockwise rotations
        self.rotate_face_cw(face);
        self.rotate_face_cw(face);
        self.rotate_face_cw(face);
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

        // Check orientations
        for y in 0..self.size {
            for x in 0..self.size {
                for z in 0..self.size {
                    if !self.orientations[y][x][z].is_aligned() {
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

    pub fn meets_difficulty(&self, target_hash: &[u8; 32]) -> bool {
        let mut hasher = Keccak::v256();
        let mut hash = [0u8; 32];

        // Create a string representation of the cube state
        let cube_state = format!("{:?}", self.faces);

        hasher.update(cube_state.as_bytes());
        hasher.finalize(&mut hash);

        // Compare the hash with the target
        // This is a simplified comparison
        &hash[..] <= target_hash
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
    Uw(usize),  // Up wide (U and M)
    Dw(usize),  // Down wide (D and S)
    Lw(usize),  // Left wide (L and M)
    Rw(usize),  // Right wide (R and M)
    Fw(usize),  // Front wide (F and S)
    Bw(usize),  // Back wide (B and S)
    X(usize),   // Entire cube rotation around X axis
    Y(usize),   // Entire cube rotation around Y axis
    Z(usize),   // Entire cube rotation around Z axis
}

impl Move {
    pub fn from_face_and_count(face: Face, count: usize) -> Self {
        match face {
            Face::Up => Move::U(count),
            Face::Down => Move::D(count),
            Face::Left => Move::L(count),
            Face::Right => Move::R(count),
            Face::Front => Move::F(count),
            Face::Back => Move::B(count),
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
        _ => {
            // For n > 3, we use a simplified approximation
            // The actual number of states for n>3 is much more complex to calculate
            (n * n * n * 24) as u32 // Simplified approximation
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}