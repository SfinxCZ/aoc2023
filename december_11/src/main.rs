mod galaxy;

use std::time::Instant;
use clap::{Parser, ValueEnum};
use crate::galaxy::parse_file;

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

fn task_1(input_file: &str) -> u64 {
    let galaxies = parse_file(input_file, 1);
    let mut distances = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            distances += galaxies[i].distance(&galaxies[j]);
        }
    }
    distances
}

fn task_2(input_file: &str) -> u64 {
    let galaxies = parse_file(input_file, 1000000);
    let mut distances = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            distances += galaxies[i].distance(&galaxies[j]);
        }
    }
    distances
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

    #[test]
    fn test_task_1() {
        assert_eq!(task_1("inputs/input_test_1.txt"), 374);
    }

    #[test]
    fn test_task_2() {
        assert_eq!(task_2("inputs/input_test_1.txt"), 1030);
    }
}