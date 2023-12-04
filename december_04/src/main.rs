use std::collections::HashSet;

use clap::{Parser, ValueEnum};
use std::fs::read_to_string;
use std::time::Instant;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Tasks {
    Task1,
    Task2,
}

#[derive(Debug, Parser)]
struct Cli {
    /// Input file to read
    input_file: String,
    /// Number of lines to read
    #[structopt(short = 't')]
    task: Tasks,
}

fn compute_numbers(line: &str) -> u32 {
    let (_, numbers) = line.split_once(":").unwrap();
    let (having_numbers_str, winning_numbers_str) = numbers.split_once("|").unwrap();
    let having_numbers = having_numbers_str
        .split_whitespace()
        .collect::<HashSet<&str>>();
    let winning_numbers = winning_numbers_str
        .split_whitespace()
        .collect::<HashSet<&str>>();
    having_numbers.intersection(&winning_numbers).count() as u32
}

fn task_1(input_file: &str) -> u32 {
    read_to_string(input_file)
        .unwrap()
        .lines()
        .map(compute_numbers)
        .map(|num| match num {
            0 => 0,
            1.. => 1 << (num - 1),
        })
        .sum()
}

fn task_2(input_file: &str) -> u32 {
    let numbers = read_to_string(input_file)
        .unwrap()
        .lines()
        .map(compute_numbers)
        .collect::<Vec<u32>>();
    let mut won_cards = vec![1; numbers.len()];
    for (i, num) in numbers.iter().enumerate() {
        let increment = won_cards[i];
        for v in &mut won_cards[(i + 1usize)..(i + (*num as usize) + 1usize)] {
            *v += increment.clone();
        }
    }
    won_cards.iter().sum()
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
    fn process_line() {
        assert_eq!(
            compute_numbers("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            4
        );
        assert_eq!(
            compute_numbers("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            2
        );
        assert_eq!(
            compute_numbers("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            2
        );
        assert_eq!(
            compute_numbers("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            1
        );
        assert_eq!(
            compute_numbers("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
        assert_eq!(
            compute_numbers("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            0
        );
    }

    #[test]
    fn test_task_1() {
        assert_eq!(task_1("./inputs/input_test.txt"), 13);
    }

    #[test]
    fn test_task_2() {
        assert_eq!(task_2("./inputs/input_test.txt"), 30);
    }

}
