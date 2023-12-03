mod parser;

use crate::parser::{parse_line, Number, Symbol, GEAR_PATTERN, NUMBER_PATTERN, SYMBOL_PATTERN};
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

fn task_1(input_file: &str) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    for (line_no, line) in read_to_string(input_file).unwrap().lines().enumerate() {
        numbers.append(&mut parse_line(line, line_no, &NUMBER_PATTERN));
        symbols.append(&mut parse_line(line, line_no, &SYMBOL_PATTERN));
    }

    let numbers_close_to_symbols = numbers
        .iter()
        .filter(|&number| {
            symbols
                .iter()
                .any(|s| number.is_close_to(s.line_no(), s.position()))
        })
        .map(|number| number.number())
        .collect::<Vec<u32>>();
    println!("{}", numbers_close_to_symbols.iter().sum::<u32>());
}

fn task_2(input_file: &str) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    for (line_no, line) in read_to_string(input_file).unwrap().lines().enumerate() {
        numbers.append(&mut parse_line(line, line_no, &NUMBER_PATTERN));
        symbols.append(&mut parse_line(line, line_no, &GEAR_PATTERN));
    }

    let close_numbers = symbols
        .iter()
        .filter_map(|symbol| {
            let close_numbers = numbers
                .iter()
                .filter(|&n| n.is_close_to(symbol.line_no(), symbol.position()))
                .collect::<Vec<&Number>>();
            return if close_numbers.len() == 2 {
                Some(close_numbers.iter().map(|&n| n.number()).product())
            } else {
                None
            };
        })
        .collect::<Vec<u32>>();
    println!("{:?}", close_numbers.iter().sum::<u32>());
}

fn main() {
    let args = Cli::parse();

    let start = Instant::now();
    match args.task {
        Tasks::Task1 => task_1(&args.input_file),
        Tasks::Task2 => task_2(&args.input_file),
    }
    println!("The task took {}ms to complete", start.elapsed().as_nanos() as f32/1_000_000.0);
}
