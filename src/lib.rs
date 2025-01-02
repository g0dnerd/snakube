use std::hash::Hash;

pub mod search;

const ZERO_POS: Position = Position { x: 0, y: 0, z: 0 };

#[derive(Default, Copy, Clone)]
pub struct Bounds {
    // Stores bounds in each cardinal direction
    // The following indices are used:
    // 0: UP
    // 1: DOWN
    // 2: RIGHT
    // 3: LEFT
    // 4: OUT
    // 5: IN
    values: [i8; 6],
}

impl Bounds {
    pub fn get_by_index(&self, idx: usize) -> i8 {
        self.values[idx]
    }

    pub fn set_by_idx(&mut self, idx: usize, value: i8, sign: i8) {
        if (value - self.values[idx]) * sign > 0 {
            self.values[idx] = value;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bitmask {
    // Stores visited coordinates in by masking them into 64-bit integers
    size: usize,
    bits: Vec<u64>,
}

impl Bitmask {
    pub fn new(size: usize) -> Self {
        // There is extra padding on the bit count because we later
        // need to pad negative coordinates into positive space
        let bit_count = (size + size - 1).pow(3);
        let word_count = (bit_count + 63) / 64;
        Self {
            size,
            bits: vec![0; word_count],
        }
    }

    fn get_index(&self, pos: Position) -> (usize, u64) {
        // Pad coordinates into known positive space
        let padded_x = (pos.x + self.size as i8) as usize;
        let padded_y = (pos.y + self.size as i8) as usize;
        let padded_z = (pos.z + self.size as i8) as usize;

        // Index for (x, y, z) and size n = (xn^2 + yn + z)/64
        let shift = self.size.trailing_zeros();
        let idx = (padded_x << (2 * shift)) + (padded_y << shift) + padded_z;
        let word_idx = idx >> 6;
        let bit_idx = idx & 0x3F;
        (word_idx, 1 << bit_idx)
    }

    pub fn is_visited(&self, pos: Position) -> bool {
        let (word_idx, bit_mask) = self.get_index(pos);
        (self.bits[word_idx] & bit_mask) != 0
    }

    pub fn mark_visited(&mut self, pos: Position) {
        let (word_idx, bit_mask) = self.get_index(pos);
        self.bits[word_idx] |= bit_mask;
    }

    pub fn backtrack(&mut self, bits: Vec<u64>) {
        self.bits = bits;
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Position {
    pub fn coord_by_dir_idx(&self, dir_idx: usize) -> i8 {
        match dir_idx {
            0 | 1 => self.y,
            2 | 3 => self.x,
            4 | 5 => self.z,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Self { x, y, z }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Direction {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Direction {
    pub fn sign(&self) -> i8 {
        self.x + self.y + self.z
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.x * 3 + self.y * 2 + self.z {
            2 => write!(f, "U"),  // UP
            -2 => write!(f, "D"), // DOWN
            3 => write!(f, "R"),  // RIGHT
            -3 => write!(f, "L"), // LEFT
            1 => write!(f, "O"),  // OUT
            -1 => write!(f, "I"), // IN
            _ => unreachable!(),
        }
    }
}

impl std::ops::Mul<i8> for Direction {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Self { x, y, z }
    }
}

impl std::ops::Mul<Direction> for Direction {
    type Output = i8;

    fn mul(self, rhs: Direction) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        x + y + z
    }
}

pub struct AttemptParams {
    pub input_queue: Vec<u8>,
    pub bounds: Bounds,
    pub state: Bitmask,
    pub direction: Option<Direction>,
    pub position: Position,
    pub solution: Vec<Position>,
}

const DIRECTIONS: [Direction; 6] = [
    Direction { x: 0, y: 1, z: 0 },
    Direction { x: 0, y: -1, z: 0 },
    Direction { x: 1, y: 0, z: 0 },
    Direction { x: -1, y: 0, z: 0 },
    Direction { x: 0, y: 0, z: 1 },
    Direction { x: 0, y: 0, z: -1 },
];

impl AttemptParams {
    pub fn new(input_queue: &[u8], size: usize) -> Self {
        let bounds = Bounds::default();
        let state = Bitmask::new(size);
        let direction = None;
        let position = Position { x: 0, y: 0, z: 0 };
        let solution = Vec::new();
        Self {
            input_queue: input_queue.to_owned(),
            bounds,
            state,
            direction,
            position,
            solution,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dotproduct() {
        let up = Direction { x: 0, y: 1, z: 0 };
        let down = Direction { x: 0, y: -1, z: 0 };
        let dprod = up * down;
        assert_eq!(dprod, -1);

        let right = Direction { x: 1, y: 0, z: 0 };
        assert_eq!(up * right, 0);
    }

    #[test]
    fn direction_inverse() {
        let up = Direction { x: 0, y: 1, z: 0 };
        let down = Direction { x: 0, y: -1, z: 0 };
        let inv = up * -1;
        assert_eq!(down, inv);
    }

    #[test]
    fn bitmask_indices() {
        let size: usize = 5;
        let bitmask = Bitmask::new(size);
        let mut indices = Vec::new();

        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let pos = Position {
                        x: (x as i8 - 2),
                        y: y as i8,
                        z: (z as i8 - 1),
                    };
                    let idx = bitmask.get_index(pos);
                    assert!(!indices.contains(&idx));
                    indices.push(idx);
                }
            }
        }
    }
}
