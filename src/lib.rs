use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
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

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
    pub z: i8,
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
    pub state: HashSet<Position>,
    pub direction: Option<Direction>,
    pub position: Position,
    pub solution: Vec<(char, u8, Position)>,
}

impl AttemptParams {
    pub fn new(input_queue: &[u8]) -> Self {
        let bounds = HashMap::from([
            (*DIRECTIONS.get("UP").unwrap(), 0),
            (*DIRECTIONS.get("DOWN").unwrap(), 0),
            (*DIRECTIONS.get("RIGHT").unwrap(), 0),
            (*DIRECTIONS.get("LEFT").unwrap(), 0),
            (*DIRECTIONS.get("OUT").unwrap(), 0),
            (*DIRECTIONS.get("IN").unwrap(), 0),
        ]);
        let state = HashSet::from([Position { x: 0, y: 0, z: 0 }]);
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

fn intersections<'a, T>(mut sets: impl Iterator<Item = &'a HashSet<T>>) -> HashSet<T>
where
    T: Clone + Eq + Hash + 'a,
{
    match sets.next() {
        Some(first) => sets.fold(first.clone(), |mut acc, set| {
            acc.retain(|item| set.contains(item));
            acc
        }),

        None => HashSet::new(),
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
    fn steps() {
        let element = 2;
        let pos = Position { x: 1, y: 0, z: 0 };
        let direction = *DIRECTIONS.get("UP").unwrap();
        let mut moves: HashSet<Position> = HashSet::new();
        for i in 1..element + 1 {
            moves.insert(pos + direction * i);
        }
        assert_eq!(
            moves,
            HashSet::from([Position { x: 1, y: 1, z: 0 }, Position { x: 1, y: 2, z: 0 },])
        );
    }

    #[test]
    fn intersect() {
        let pos1 = Position { x: 0, y: 0, z: 0 };
        let pos2 = Position { x: 1, y: 0, z: 0 };
        let pos3 = Position { x: 0, y: 1, z: 0 };
        let pos4 = Position { x: 0, y: 0, z: 1 };
        let a = HashSet::from([pos1, pos2]);
        let b = HashSet::from([pos3, pos4]);
        assert!(intersections([a.clone(), b.clone()].iter()).is_empty());

        let c = HashSet::from([pos1, pos3]);
        assert!(!intersections([a.clone(), c.clone()].iter()).is_empty());
    }
}
