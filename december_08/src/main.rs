use num::integer::lcm;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::fs::read_to_string;
use std::time::Instant;

use clap::{Parser, ValueEnum};
use regex::Regex;

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

trait Movable {
    fn is_finished(&self) -> bool;
    fn move_to_next(&mut self, instruction: char);
}

struct State<'a> {
    current_position: &'a str,
    end_suffix: &'a str,

    nodes: &'a HashMap<String, (String, String)>,
}

impl<'a> Movable for State<'a> {
    fn is_finished(&self) -> bool {
        self.current_position.ends_with(&self.end_suffix)
    }

    fn move_to_next(&mut self, instruction: char) {
        match instruction {
            'L' => self.current_position = &self.nodes.get(self.current_position).unwrap().0,
            'R' => self.current_position = &self.nodes.get(self.current_position).unwrap().1,
            _ => {
                panic!("Unknown instruction {}", instruction)
            }
        }
    }
}

fn parse_file(input_file: &str) -> (String, HashMap<String, (String, String)>) {
    let node_pattern = Regex::new("([0-9A-Z]+)\\s+=\\s+\\(([0-9A-Z]+),\\s+([0-9A-Z]+)\\)").unwrap();

    let content = read_to_string(input_file).unwrap();
    let mut lines = content.lines();
    let instructions = lines.next().unwrap().to_string();
    lines.next();

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let captures = node_pattern.captures(line).unwrap();
        let node_name = captures.get(1).unwrap().as_str().to_string();
        let left_node = captures.get(2).unwrap().as_str().to_string();
        let right_node = captures.get(3).unwrap().as_str().to_string();
        nodes.insert(node_name, (left_node, right_node));
    }
    (instructions, nodes)
}

fn compute_num_steps(instructions: String, state: &mut impl Movable) -> u64 {
    let mut instruction_vec: VecDeque<char> = VecDeque::new();
    let mut num_steps = 0;

    while !state.is_finished() || !instruction_vec.is_empty() {
        if instruction_vec.is_empty() {
            let mut chars = instructions.clone().chars().into_iter().collect();
            instruction_vec.append(&mut chars);
        }
        state.move_to_next(instruction_vec.pop_front().unwrap());
        num_steps += 1;
    }
    num_steps
}

fn task_1(input_file: &str) -> u64 {
    let (instructions, nodes) = parse_file(input_file);
    let mut state = State {
        end_suffix: "ZZZ",
        current_position: "AAA",
        nodes: &nodes,
    };
    compute_num_steps(instructions, &mut state)
}

fn task_2(input_file: &str) -> u64 {
    let (instructions, nodes) = parse_file(input_file);

    let mut states_vec: Vec<State> = nodes
        .keys()
        .into_iter()
        .filter(|&node| node.ends_with("A"))
        .map(|node| State {
            end_suffix: "Z",
            current_position: node,
            nodes: &nodes,
        })
        .collect::<Vec<State>>();

    states_vec
        .iter_mut()
        .map(|state| compute_num_steps(instructions.clone(), state))
        .fold(1, lcm)
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
    fn test_parse_input() {
        let (instructions, nodes) = parse_file("./inputs/input_test_1.txt");

        let exp_nodes: HashMap<String, (String, String)> = HashMap::from([
            ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string())),
            ("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string())),
            ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
        ]);
        assert_eq!(instructions, "LLR");
        assert_eq!(nodes, exp_nodes);
    }

    #[test]
    fn test_compute_num_steps() {
        let nodes: HashMap<String, (String, String)> = HashMap::from([
            ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string())),
            ("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string())),
            ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
        ]);
        let instructions = "LLR".to_string();
        let mut state = State {
            current_position: "AAA",
            end_suffix: "ZZZ",
            nodes: &nodes,
        };
        assert_eq!(compute_num_steps(instructions, &mut state), 6);
    }

    #[test]
    fn test_task_1() {
        assert_eq!(task_1("./inputs/input_test_1.txt"), 6);
        assert_eq!(task_1("./inputs/input_test_2.txt"), 2);
    }

    #[test]
    fn test_task_2() {
        assert_eq!(task_2("./inputs/input_test_3.txt"), 6);
    }
}
