use ndarray::{Array2, ArrayViewMut2};
use std::collections::HashSet;
use std::ops::Add;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Next<T> {
    Single(T),
    Split(T, T),
}

impl Direction {
    fn as_vector(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }

    fn next_direction(&self, current_char: char) -> Next<Direction> {
        match (self, current_char) {
            (Direction::East, '/') => Next::Single(Direction::North),
            (Direction::North, '/') => Next::Single(Direction::East),
            (Direction::West, '/') => Next::Single(Direction::South),
            (Direction::South, '/') => Next::Single(Direction::West),
            (Direction::North, '\\') => Next::Single(Direction::West),
            (Direction::South, '\\') => Next::Single(Direction::East),
            (Direction::East, '\\') => Next::Single(Direction::South),
            (Direction::West, '\\') => Next::Single(Direction::North),
            (&dir, '-') if dir == Direction::East || dir == Direction::West => Next::Single(dir),
            (&dir, '|') if dir == Direction::North || dir == Direction::South => Next::Single(dir),
            (&dir, '-') if dir == Direction::South || dir == Direction::North => {
                Next::Split(Direction::West, Direction::East)
            }
            (&dir, '|') if dir == Direction::East || dir == Direction::West => {
                Next::Split(Direction::North, Direction::South)
            }
            (dir, '.') => Next::Single(dir.clone()),
            (dir, char) => panic!("Unknown combination of {:?} and {:?}", dir, char),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn row(&self) -> usize {
        self.0
    }

    pub fn col(&self) -> usize {
        self.1
    }

    pub fn is_in_bounds(&self, shape: &[usize]) -> bool {
        self.row() < shape[0] && self.col() < shape[1]
    }

    pub fn index_tuple(&self) -> (usize, usize) {
        (self.row(), self.col())
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        let vec = rhs.as_vector();
        Position(
            (self.0 as isize + vec.0) as usize,
            (self.1 as isize + vec.1) as usize,
        )
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct BeamPosition {
    position: Position,
    direction: Direction, // inbound direction
}

impl BeamPosition {
    pub fn new() -> Self {
        BeamPosition {
            position: Position(0, 0),
            direction: Direction::East,
        }
    }

    pub fn start(position: Position, direction: Direction) -> Self {
        BeamPosition {position, direction}
    }

    pub fn from(
        position: Position,
        next_direction: Direction,
        mirrors_shape: &[usize],
    ) -> Option<Self> {
        let next_position = position + next_direction;
        if next_position.is_in_bounds(mirrors_shape) {
            Some(BeamPosition {
                position: next_position,
                direction: next_direction,
            })
        } else {
            None
        }
    }

    pub fn next_beam_position(&self, mirrors_pattern: &Array2<char>) -> Next<Option<BeamPosition>> {
        let current_char = mirrors_pattern[self.position.index_tuple()];
        let next = self.direction.next_direction(current_char);
        return match next {
            Next::Single(dir) => Next::Single(BeamPosition::from(
                self.position,
                dir,
                mirrors_pattern.shape(),
            )),
            Next::Split(dir_1, dir_2) => Next::Split(
                BeamPosition::from(self.position, dir_1, mirrors_pattern.shape()),
                BeamPosition::from(self.position, dir_2, mirrors_pattern.shape()),
            ),
        };
    }
}

pub fn move_beam(
    beam: BeamPosition,
    mirrors_pattern: &Array2<char>,
    output: &mut ArrayViewMut2<u64>,
    visited_positions: &mut HashSet<BeamPosition>,
) {
    if visited_positions.contains(&beam) {
        return;
    } else {
        visited_positions.insert(beam.clone());
    }
    output[beam.position.index_tuple()] = 1;

    let next = beam.next_beam_position(mirrors_pattern);
    match next {
        Next::Single(maybe_next_beam) => {
            if let Some(next_beam) = maybe_next_beam {
                move_beam(next_beam, mirrors_pattern, output, visited_positions);
            }
        }
        Next::Split(maybe_next_beam_1, maybe_next_beam_2) => {
            if let Some(next_beam) = maybe_next_beam_1 {
                move_beam(next_beam, mirrors_pattern, output, visited_positions);
            }
            if let Some(next_beam) = maybe_next_beam_2 {
                move_beam(next_beam, mirrors_pattern, output, visited_positions);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_direction_dot() {
        for d in vec![
            Direction::East,
            Direction::West,
            Direction::North,
            Direction::South,
        ] {
            assert_eq!(Next::Single(d), d.next_direction('.'))
        }
    }

    #[test]
    fn test_next_direction_pipe() {
        for d in vec![Direction::North, Direction::South] {
            assert_eq!(Next::Single(d), d.next_direction('|'))
        }

        assert_eq!(
            Next::Split(Direction::North, Direction::South),
            Direction::West.next_direction('|')
        );
        assert_eq!(
            Next::Split(Direction::North, Direction::South),
            Direction::East.next_direction('|')
        );
    }

    #[test]
    fn test_next_direction_dash() {
        for d in vec![Direction::East, Direction::West] {
            assert_eq!(Next::Single(d), d.next_direction('-'))
        }
        assert_eq!(
            Next::Split(Direction::West, Direction::East),
            Direction::South.next_direction('-')
        );
        assert_eq!(
            Next::Split(Direction::West, Direction::East),
            Direction::North.next_direction('-')
        );
    }

    #[test]
    fn test_next_direction_slash() {
        assert_eq!(
            Next::Single(Direction::North),
            Direction::East.next_direction('/')
        );
        assert_eq!(
            Next::Single(Direction::West),
            Direction::South.next_direction('/')
        );
        assert_eq!(
            Next::Single(Direction::South),
            Direction::West.next_direction('/')
        );
        assert_eq!(
            Next::Single(Direction::West),
            Direction::South.next_direction('/')
        );
    }

    #[test]
    fn test_next_direction_backslash() {
        assert_eq!(
            Next::Single(Direction::North),
            Direction::West.next_direction('\\')
        );
        assert_eq!(
            Next::Single(Direction::East),
            Direction::South.next_direction('\\')
        );
        assert_eq!(
            Next::Single(Direction::South),
            Direction::East.next_direction('\\')
        );
        assert_eq!(
            Next::Single(Direction::West),
            Direction::North.next_direction('\\')
        );
    }

    #[test]
    fn test_position_is_in_bounds() {
        assert!(Position(0usize, 0usize).is_in_bounds(&[10usize, 10usize]));
        assert!(Position(0usize, 9usize).is_in_bounds(&[10usize, 10usize]));
        assert!(Position(9usize, 9usize).is_in_bounds(&[10usize, 10usize]));
        assert!(!Position(11usize, 0usize).is_in_bounds(&[10usize, 10usize]));
        assert!(!Position(11usize, 11usize).is_in_bounds(&[10usize, 10usize]));
        assert!(!Position(0usize, 11usize).is_in_bounds(&[10usize, 10usize]));
    }
}
