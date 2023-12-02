use std::fs::read_to_string;

mod digit_parser;

fn main() {
    let matcher = crate::digit_parser::Matcher::default();
    let sum_of_digits: u32 = read_to_string("./input/input_part1.txt")
        .unwrap()
        .lines()
        .map(|line| matcher.find_coordinates(line))
        .sum();
    println!("{}", sum_of_digits)
}
