mod task_1;
mod task_2;

use clap::{Parser, ValueEnum};
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

fn main() {
    let args = Cli::parse();

    let start = Instant::now();
    let result = match args.task {
        Tasks::Task1 => crate::task_1::run(&args.input_file),
        Tasks::Task2 => crate::task_2::run(&args.input_file),
    };
    println!(
        "The task took {}ms to complete",
        start.elapsed().as_nanos() as f32 / 1_000_000.0
    );
    println!("Result is {}", result);
}
