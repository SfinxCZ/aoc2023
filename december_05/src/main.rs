use clap::{Parser, ValueEnum};
use rayon::prelude::*;
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

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct SingleMapping {
    dest_range_start: i64,
    src_range_start: i64,
    range_len: i64,
}

impl SingleMapping {
    pub fn new(dest_range_start: i64, src_range_start: i64, range_len: i64) -> Self {
        SingleMapping {
            dest_range_start,
            src_range_start,
            range_len,
        }
    }

    pub fn src_range_start(&self) -> i64 {
        self.src_range_start
    }

    pub fn src_range_end(&self) -> i64 {
        self.src_range_start + self.range_len
    }

    pub fn map(&self, source: i64) -> i64 {
        source - self.src_range_start + self.dest_range_start
    }
}

impl From<&str> for SingleMapping {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        SingleMapping::new(
            parts.next().unwrap().parse::<i64>().unwrap().to_owned(),
            parts.next().unwrap().parse::<i64>().unwrap().to_owned(),
            parts.next().unwrap().parse::<i64>().unwrap().to_owned(),
        )
    }
}

struct Mapping {
    id: String,
    mappings: Vec<SingleMapping>,
}

impl From<&str> for Mapping {
    fn from(chunk: &str) -> Self {
        let mut lines = chunk.lines();
        let id = lines.next().unwrap().to_owned();
        let mut mappings = lines
            .map(SingleMapping::from)
            .collect::<Vec<SingleMapping>>();
        Mapping { id, mappings }
    }
}

impl Mapping {
    pub fn map(&self, source: i64) -> i64 {
        let maybe_mapping = self
            .mappings
            .iter()
            .find(|&m| (m.src_range_start() <= source) && (source < m.src_range_end()));
        match maybe_mapping {
            None => source,
            Some(mapping) => mapping.map(source),
        }
    }
}

fn map(source: i64, mappings: &Vec<Mapping>) -> i64 {
    let mut target = source;
    for mapping in mappings {
        target = mapping.map(target);
    }
    target
}

fn task_1(input_file: &str) -> i64 {
    let file_content = read_to_string(input_file).unwrap();
    let mut chunks = file_content.split("\n\n");
    let seeds_str = chunks.next().unwrap().split_once(":").unwrap().1;

    let mappings = chunks.map(Mapping::from).collect::<Vec<Mapping>>();

    seeds_str
        .split_whitespace()
        .map(|seed_str| seed_str.parse().unwrap())
        .map(|seed| map(seed, &mappings))
        .min()
        .unwrap()
}

fn task_2(input_file: &str) -> i64 {
    let file_content = read_to_string(input_file).unwrap();
    let mut chunks = file_content.split("\n\n");
    let seeds_str = chunks.next().unwrap().split_once(":").unwrap().1;

    let mappings = chunks.map(Mapping::from).collect::<Vec<Mapping>>();

    let seeds = seeds_str
        .split_whitespace()
        .map(|seed_str| seed_str.parse().unwrap())
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|c| Vec::from(c))
        .collect::<Vec<Vec<i64>>>();

    seeds
        .par_iter()
        .flat_map(|seed_range| {
            println!("Processing range {:?}", seed_range);
            seed_range[0]..(seed_range[0] + seed_range[1])
        })
        .map(|seed| map(seed, &mappings))
        .min()
        .unwrap()
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
    fn test_mapping_map_1() {
        let single_mappings = vec![SingleMapping::new(50, 98, 2)];
        let mapping = Mapping {
            id: "abc".to_string(),
            mappings: single_mappings,
        };

        assert_eq!(mapping.map(95), 95);
        assert_eq!(mapping.map(96), 96);
        assert_eq!(mapping.map(97), 97);
        assert_eq!(mapping.map(98), 50);
        assert_eq!(mapping.map(99), 51);
        assert_eq!(mapping.map(100), 100);
    }

    #[test]
    fn test_mapping_map_2() {
        let single_mappings = vec![
            SingleMapping::new(49, 53, 8),
            SingleMapping::new(0, 11, 42),
            SingleMapping::new(42, 0, 7),
            SingleMapping::new(57, 7, 4),
        ];
        let mapping = Mapping {
            id: "abc".to_string(),
            mappings: single_mappings,
        };

        assert_eq!(mapping.map(81), 81);
        assert_eq!(mapping.map(53), 49);
        assert_eq!(mapping.map(57), 53);
        assert_eq!(mapping.map(52), 41);
    }

    #[test]
    fn test_map() {
        let mapping = Mapping {
            id: "abc".to_string(),
            mappings: vec![
                SingleMapping::new(50, 98, 2),
                SingleMapping::new(52, 50, 48),
            ],
        };
        assert_eq!(mapping.map(0), 0);
        assert_eq!(mapping.map(20), 20);
        assert_eq!(mapping.map(40), 40);
        assert_eq!(mapping.map(50), 52);
        assert_eq!(mapping.map(60), 62);
        assert_eq!(mapping.map(97), 99);
        assert_eq!(mapping.map(98), 50);
        assert_eq!(mapping.map(99), 51);
        assert_eq!(mapping.map(100), 100);
        assert_eq!(mapping.map(110), 110);
    }

    #[test]
    fn test_task_1() {
        assert_eq!(task_1("./inputs/input_test.txt"), 35);
    }

    #[test]
    fn test_task_2() {
        assert_eq!(task_2("./inputs/input_test.txt"), 46);
    }
}
