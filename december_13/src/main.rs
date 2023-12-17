use std::fs::read_to_string;
use std::time::Instant;

use clap::{Parser, ValueEnum};
use ndarray::{Array2, Axis};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tasks {
    Task1,
    Task2,
}

#[derive(Debug, Parser)]
pub struct Cli {
    /// Input file to read
    input_file: String,
    /// Number of lines to read
    #[structopt(short = 't')]
    task: Tasks,
}

fn parse_pattern(data: &str) -> Array2<char> {
    let chars = data
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let nrows = chars.len();
    let ncols = chars[0].len();

    Array2::from_shape_vec((nrows, ncols), chars.iter().flatten().map(|&c| c).collect()).unwrap()
}

fn is_mirror(array: &Array2<char>, start: usize, axis: Axis) -> bool {
    if start + 1 >= array.shape()[axis.index()] {
        return false;
    }
    for (i, j) in (0..=start)
        .rev()
        .zip((start + 1)..array.shape()[axis.index()])
    {
        if array.index_axis(axis, i) != array.index_axis(axis, j) {
            return false;
        }
    }
    return true;
}

fn count_diffs(array: &Array2<char>, start: usize, axis: Axis) -> Option<u64> {
    if start + 1 >= array.shape()[axis.index()] {
        return None;
    }
    let mut diff: u64 = 0;
    for (i, j) in (0..=start)
        .rev()
        .zip((start + 1)..array.shape()[axis.index()])
    {
        let row_i = array.index_axis(axis, i);
        let row_j = array.index_axis(axis, j);
        diff += row_i
            .iter()
            .zip(row_j)
            .map(|(e_i, e_j)| if e_i == e_j { 0 } else { 1 })
            .sum::<u64>();
    }
    return Some(diff);
}

fn find_index(
    array: &Array2<char>,
    axis: Axis,
    predicate: &dyn Fn(&Array2<char>, usize, Axis) -> bool,
) -> Option<usize> {
    (0..array.len_of(axis))
        .into_iter()
        .find(|&i| predicate(array, i, axis))
}

fn compute_score(maybe_index: Option<usize>) -> u64 {
    match maybe_index {
        None => 0u64,
        Some(i) => i as u64 + 1,
    }
}

fn task_1(input_file: &str) -> u64 {
    let file_content = read_to_string(input_file).unwrap();
    let patterns = file_content.trim().split("\n\n").collect::<Vec<&str>>();
    let arrays = patterns
        .iter()
        .map(|&s| parse_pattern(s))
        .collect::<Vec<Array2<char>>>();


    let rows: u64 = arrays
        .clone()
        .into_iter()
        .map(|a| compute_score(find_index(&a, Axis(0), &is_mirror)))
        .sum();
    let cols: u64 = arrays
        .clone()
        .into_iter()
        .map(|a| compute_score(find_index(&a, Axis(1), &is_mirror)))
        .sum();
    rows * 100 + cols
}

fn task_2(input_file: &str) -> u64 {
    let file_content = read_to_string(input_file).unwrap();
    let patterns = file_content.trim().split("\n\n").collect::<Vec<&str>>();
    let arrays = patterns
        .iter()
        .map(|&s| parse_pattern(s))
        .collect::<Vec<Array2<char>>>();

    let predicate = |arr: &Array2<char>, start, axis| count_diffs(&arr, start, axis).unwrap_or(0) == 1;

    let rows: u64 = arrays
        .clone()
        .into_iter()
        .map(|a| compute_score(find_index(&a, Axis(0), &predicate)))
        .sum();
    let cols: u64 = arrays
        .clone()
        .into_iter()
        .map(|a| compute_score(find_index(&a, Axis(1), &predicate)))
        .sum();
    rows * 100 + cols
}

fn main() {
    let args = Cli::parse();

    let start = Instant::now();
    let result = match args.task {
        Tasks::Task1 => task_1(&args.input_file),
        Tasks::Task2 => task_2(&args.input_file),
    };
    println!(
        "The task took {}ms to complete",
        start.elapsed().as_nanos() as f32 / 1_000_000.0
    );
    println!("Result is {}", result);
}

#[cfg(test)]
mod tests {
    use ndarray::{array, Axis};

    use crate::{find_index, is_mirror, parse_pattern, task_1, task_2};

    #[test]
    fn test_parse_pattern() {
        let data = vec![
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
        ]
        .join("\n");
        let array = parse_pattern(&data);
        assert_eq!(7, array.nrows());
        assert_eq!(9, array.ncols());
    }

    #[test]
    fn test_is_mirror() {
        let data = array![
            ['#', '.', '#', '#', '.', '.', '#', '#', '.'],
            ['.', '.', '#', '.', '#', '#', '.', '#', '.'],
            ['#', '#', '.', '.', '.', '.', '.', '.', '#'],
            ['#', '#', '.', '.', '.', '.', '.', '.', '#'],
            ['.', '.', '#', '.', '#', '#', '.', '#', '.'],
            ['.', '.', '#', '#', '.', '.', '#', '#', '.'],
            ['#', '.', '#', '.', '#', '#', '.', '#', '.']
        ];
        assert!(!is_mirror(&data, 0, Axis(0)));
        assert!(!is_mirror(&data, 1, Axis(0)));
        assert!(!is_mirror(&data, 2, Axis(0)));
        assert!(!is_mirror(&data, 3, Axis(0)));
        assert!(!is_mirror(&data, 4, Axis(0)));
        assert!(!is_mirror(&data, 5, Axis(0)));
        assert!(!is_mirror(&data, 6, Axis(0)));

        assert!(!is_mirror(&data, 0, Axis(1)));
        assert!(!is_mirror(&data, 1, Axis(1)));
        assert!(!is_mirror(&data, 2, Axis(1)));
        assert!(!is_mirror(&data, 3, Axis(1)));
        assert!(is_mirror(&data, 4, Axis(1)));
        assert!(!is_mirror(&data, 5, Axis(1)));
        assert!(!is_mirror(&data, 6, Axis(1)));
        assert!(!is_mirror(&data, 7, Axis(1)));
        assert!(!is_mirror(&data, 8, Axis(1)));
    }

    #[test]
    fn test_is_mirror_2() {
        let data = array![
            ['.', '.', '#', '#', '#', '.', '.', '#', '#', '.', '.'],
            ['.', '#', '.', '#', '.', '.', '.', '.', '.', '#', '#'],
            ['#', '.', '#', '#', '.', '.', '#', '#', '.', '#', '.'],
            ['#', '.', '#', '.', '.', '#', '#', '#', '.', '.', '#'],
            ['.', '#', '.', '.', '#', '#', '.', '#', '#', '.', '#'],
            ['.', '#', '.', '.', '.', '.', '#', '#', '.', '.', '#'],
            ['#', '.', '#', '#', '#', '#', '.', '#', '#', '.', '#'],
            ['#', '.', '#', '#', '#', '#', '.', '#', '#', '.', '#'],
            ['.', '#', '.', '.', '.', '.', '#', '#', '#', '.', '#'],
            ['.', '#', '.', '.', '#', '#', '.', '#', '#', '.', '#'],
            ['#', '.', '#', '.', '.', '#', '#', '#', '.', '.', '#'],
            ['#', '.', '#', '#', '.', '.', '#', '#', '.', '#', '.'],
            ['.', '#', '.', '#', '.', '.', '.', '.', '.', '#', '#'],
            ['.', '.', '#', '#', '#', '.', '.', '#', '#', '.', '.'],
            ['.', '.', '#', '#', '#', '.', '.', '#', '#', '.', '.']
        ];

        assert!(!is_mirror(&data, 11, Axis(1)));
    }

    #[test]
    fn test_is_mirror_3() {
        let data = vec![
            "##.#.#..#.##.#..#",
            ".####..##....##..",
            "#.##.#.##.##.##.#",
            "#..#..#..#..#..#.",
            "###.#############",
            "#.##.............",
            "...####..####..##",
            "..#..##..####..##",
            "#.#.#.####..####.",
        ];
        assert!(is_mirror(&parse_pattern(&data.join("\n")), 10, Axis(1)));
    }

    #[test]
    fn test_find_index() {
        let data = array![
            ['#', '.', '#', '#', '.', '.', '#', '#', '.'],
            ['.', '.', '#', '.', '#', '#', '.', '#', '.'],
            ['#', '#', '.', '.', '.', '.', '.', '.', '#'],
            ['#', '#', '.', '.', '.', '.', '.', '.', '#'],
            ['.', '.', '#', '.', '#', '#', '.', '#', '.'],
            ['.', '.', '#', '#', '.', '.', '#', '#', '.'],
            ['#', '.', '#', '.', '#', '#', '.', '#', '.']
        ];
        assert_eq!(Some(4), find_index(&data, Axis(1), &is_mirror))
    }

    #[test]
    fn test_find_index_2() {
        let data = vec![
            "##.#.#..#.##.#..#",
            ".####..##....##..",
            "#.##.#.##.##.##.#",
            "#..#..#..#..#..#.",
            "###.#############",
            "#.##.............",
            "...####..####..##",
            "..#..##..####..##",
            "#.#.#.####..####.",
        ];
        assert_eq!(
            Some(10),
            find_index(&parse_pattern(&data.join("\n")), Axis(1), &is_mirror)
        );
    }

    #[test]
    fn test_task_1() {
        assert_eq!(405, task_1("./inputs/input_test_1.txt"));
    }

    #[test]
    fn test_task_2() {
        assert_eq!(400, task_2("./inputs/input_test_1.txt"));
    }
}
