mod hashmap;
mod task_2;

use crate::hashmap::LinkedHashMap;
use crate::task_2::{parse_element, Operation};
use clap::{Parser, ValueEnum};
use std::fs::read_to_string;
use std::time::Instant;

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

fn compute_hash(string: &str) -> u64 {
    let mut current_value: u64 = 0;
    string.chars().for_each(|ch| {
        current_value += ch as u64;
        current_value *= 17;
        current_value = current_value % 256;
    });
    current_value
}

fn task_1(input_file: &str) -> u64 {
    let raw_data = read_to_string(input_file).unwrap();
    let data = raw_data.split_whitespace().collect::<String>();
    let parts = data.split(",");
    parts.map(compute_hash).sum()
}

fn task_2(input_file: &str) -> u64 {
    let raw_data = read_to_string(input_file).unwrap();
    let data = raw_data.split_whitespace().collect::<String>();
    let parts = data.split(",");

    let mut map: LinkedHashMap<String, u64> = LinkedHashMap::new();

    parts.map(parse_element).for_each(|(name, op)| {
        let name_str = name.clone();
        match op {
            Operation::Insert(num) => map.insert(name_str, num),
            Operation::Remove => map.remove(name_str),
        };
    });

    map.data()
        .iter()
        .enumerate()
        .map(|(box_id, lenses)| {
            lenses
                .values()
                .enumerate()
                .map(|(lens_id, lens_val)| (box_id + 1) as u64 * (lens_id + 1) as u64 * lens_val)
                .sum::<u64>()
        })
        .sum()
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
    fn test_compute_hash() {
        assert_eq!(52, compute_hash(&"HASH"));
        assert_eq!(30, compute_hash(&"rn=1"));
        assert_eq!(253, compute_hash(&"cm-"));
    }

    #[test]
    fn test_task_1() {
        assert_eq!(1320, task_1("./inputs/input_test_1.txt"))
    }

    #[test]
    fn test_task_2() {
        assert_eq!(145, task_2("./inputs/input_test_1.txt"))
    }
}
