mod commons;
mod io;

use std::collections::HashSet;
use std::fs::read_to_string;
use aoc_2023::run_main;
use ndarray::Array2;
use crate::commons::{BeamPosition, Direction, Position};
use crate::io::parse_pattern;


fn compute_coverage(mirrors_pattern: &Array2<char>, beam: BeamPosition) -> u64 {
    let mut output: Array2<u64> = Array2::from_elem(mirrors_pattern.dim(), 0);
    let mut visited_positions: HashSet<BeamPosition> = HashSet::new();

    commons::move_beam(
        beam,
        &mirrors_pattern,
        &mut output.view_mut(),
        &mut visited_positions,
    );
    output.sum()
}

pub fn task_1(input_file: &str) -> u64 {
    let data = read_to_string(input_file).unwrap();
    let mirrors_pattern = parse_pattern(data.as_str());
    compute_coverage(&mirrors_pattern, BeamPosition::new())
}


fn task_2(input_file: &str) -> u64 {
    let data = read_to_string(input_file).unwrap();
    let mirrors_pattern = parse_pattern(data.as_str());

    let mut positions: Vec<BeamPosition> = Vec::new();
    for row in 0..mirrors_pattern.nrows() {
        positions.push(BeamPosition::start(Position(row, 0), Direction::East));
        positions.push(BeamPosition::start(Position(row, mirrors_pattern.ncols() - 1), Direction::West));
    }
    for col in 0..mirrors_pattern.nrows() {
        positions.push(BeamPosition::start(Position(0, col), Direction::South));
        positions.push(BeamPosition::start(Position(mirrors_pattern.nrows() - 1, col), Direction::North));
    }

    positions.iter().map(|&start_pos| compute_coverage(&mirrors_pattern, start_pos)).max().unwrap()
}

fn main() {
    run_main(task_1, task_2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_1() {
        assert_eq!(46, task_1("./inputs/input_test_1.txt"));
    }

    #[test]
    fn test_task_2() {
        assert_eq!(51, task_2("./inputs/input_test_1.txt"));
    }
}