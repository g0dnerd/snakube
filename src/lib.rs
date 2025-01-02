use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

pub mod search;

lazy_static! {
    pub static ref DIRECTIONS: HashMap<&'static str, Direction> = HashMap::from([
        ("UP", Direction { x: 0, y: 1, z: 0 }),
        ("DOWN", Direction { x: 0, y: -1, z: 0 }),
        ("RIGHT", Direction { x: 1, y: 0, z: 0 }),
        ("LEFT", Direction { x: -1, y: 0, z: 0 }),
        ("OUT", Direction { x: 0, y: 0, z: 1 }),
        ("IN", Direction { x: 0, y: 0, z: -1 }),
    ]);
}

const ZERO_POS: Position = Position { x: 0, y: 0, z: 0 };

#[derive(Debug, Clone)]
pub struct Bitmask {
    size: usize,
    bits: Vec<u64>,
}

impl Bitmask {
    pub fn new(size: usize) -> Self {
        // Have extra padding on the bit count because we later
        // need to pad negative coordinates into positive space
        let bit_count = (size + size - 1).pow(3);
        let word_count = (bit_count + 63) / 64;
        Self {
            size,
            bits: vec![0; word_count],
        }
    }

    fn get_index(&self, pos: Position) -> (usize, u64) {
        let padded_x = (pos.x + self.size as i8) as usize;
        let padded_y = (pos.y + self.size as i8) as usize;
        let padded_z = (pos.z + self.size as i8) as usize;
        let idx = padded_x * self.size * self.size + padded_y * self.size + padded_z;
        let word_idx = idx / 64;
        let bit_idx = idx % 64;
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
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Position {
    pub fn coord_by_dir(&self, dir: &str) -> i8 {
        match dir {
            "UP" | "DOWN" => self.y,
            "LEFT" | "RIGHT" => self.x,
            "OUT" | "IN" => self.z,
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

impl Display for Position {
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

    pub fn abbreviation(&self) -> char {
        match self.x.signum() {
            1 => 'R',
            -1 => 'L',
            _ => match self.y.signum() {
                1 => 'U',
                -1 => 'D',
                _ => match self.z.signum() {
                    1 => 'O',
                    -1 => 'I',
                    _ => unreachable!(),
                },
            },
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.x.signum() {
            1 => write!(f, "RIGHT"),
            -1 => write!(f, "LEFT"),
            _ => match self.y.signum() {
                1 => write!(f, "UP"),
                -1 => write!(f, "DOWN"),
                _ => match self.z.signum() {
                    1 => write!(f, "OUT"),
                    -1 => write!(f, "IN"),
                    _ => unreachable!(),
                },
            },
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
    pub bounds: HashMap<Direction, i8>,
    pub state: Bitmask,
    pub direction: Option<Direction>,
    pub position: Position,
    pub solution: Vec<(char, u8, Position)>,
}

impl AttemptParams {
    pub fn new(input_queue: &[u8], size: usize) -> Self {
        let bounds = HashMap::from([
            (*DIRECTIONS.get("UP").unwrap(), 0),
            (*DIRECTIONS.get("DOWN").unwrap(), 0),
            (*DIRECTIONS.get("RIGHT").unwrap(), 0),
            (*DIRECTIONS.get("LEFT").unwrap(), 0),
            (*DIRECTIONS.get("OUT").unwrap(), 0),
            (*DIRECTIONS.get("IN").unwrap(), 0),
        ]);
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
        let up = *DIRECTIONS.get("UP").unwrap();
        let down = *DIRECTIONS.get("DOWN").unwrap();
        let dprod = up * down;
        assert_eq!(dprod, -1);

        let right = *DIRECTIONS.get("RIGHT").unwrap();
        assert_eq!(up * right, 0);
    }

    #[test]
    fn direction_inverse() {
        let up = *DIRECTIONS.get("UP").unwrap();
        let down = *DIRECTIONS.get("DOWN").unwrap();
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
