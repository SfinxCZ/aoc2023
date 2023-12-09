use clap::{Parser, ValueEnum};
use ndarray::{s, Array1};
use std::fs::read_to_string;
use std::time::Instant;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tasks {
    Task1,
    Task2,
}

type Row = Array1<i64>;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Input file to read
    input_file: String,
    /// Number of lines to read
    #[structopt(short = 't')]
    task: Tasks,
}

fn line_to_array(line: &str) -> Row {
    line.split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect::<Row>()
}

fn find_prediction(row: Row) -> (i64, i64) {
    let diff: Row = &row.slice(s![1..]) - &row.slice(s![..-1]);
    if diff == Row::zeros(diff.len()) {
        return (row[0], row[row.len() - 1]);
    } else {
        let (first_prediction, last_prediction) = find_prediction(diff);
        (row[0] - first_prediction, row[row.len() - 1] + last_prediction)
    }
}

fn task_1(input_file: &str) -> i64 {
    let content = read_to_string(input_file).unwrap();
    let lines = content.lines();

     lines.map(line_to_array).map(|row| find_prediction(row).1).sum()
}

fn task_2(input_file: &str) -> i64 {
    let content = read_to_string(input_file).unwrap();
    let lines = content.lines();

    lines.map(line_to_array).map(|row| find_prediction(row).0).sum()
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
    use super::*;
    use ndarray::array;

    #[test]
    fn test_find_prediction() {
        assert_eq!(find_prediction(array![0, 3, 6, 9, 12, 15]), (-3, 18));
        assert_eq!(find_prediction(array![1, 3, 6, 10, 15, 21]), (0, 28));
        assert_eq!(find_prediction(array![10, 13, 16, 21, 30, 45]), (5, 68));
    }

    #[test]
    fn test_task_1() {
        assert_eq!(task_1("./inputs/input_test.txt"), 114);
    }
}
