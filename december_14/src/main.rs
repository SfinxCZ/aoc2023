use clap::{Parser, ValueEnum};
use ndarray::{s, Array2, ArrayView1, ArrayViewMut1, Axis};
use ndarray_slice::Slice1Ext;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

pub trait AocArray {
    fn parse_pattern(data: &str) -> Self;
    fn rot90(self: &mut Self);
}

impl AocArray for Array2<char> {
    fn parse_pattern(data: &str) -> Self {
        let chars = data
            .split("\n")
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let nrows = chars.len();
        let ncols = chars[0].len();

        Array2::from_shape_vec((nrows, ncols), chars.iter().flatten().map(|&c| c).collect())
            .unwrap()
    }

    fn rot90(self: &mut Self) {
        self.swap_axes(0, 1);
        self.invert_axis(Axis(1));
    }
}

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


fn sort_line(input: &mut ArrayViewMut1<char>) {
    let mut sharp_pos: Vec<usize> = vec![0];
    for (i, &e) in input.iter().enumerate() {
        if e == '#' {
            sharp_pos.push(i + 1);
        }
    }
    sharp_pos.push(input.len());
    for window in sharp_pos.windows(2) {
        // eprintln!("{}, {}", window[0], window[1]);
        // eprintln!("{:?}", input.slice(s![window[0]..window[1]]));
        input.slice_mut(s![window[0]..window[1]]).sort();
        input.slice_mut(s![window[0]..window[1]]).reverse();
        // eprintln!("{:?}", input.slice(s![window[0]..window[1]]));
    }
}

fn compute_weight(line: ArrayView1<char>) -> u64 {
    line.iter()
        .zip((1..=line.len()).rev())
        .map(|(&ch, line_no)| if ch == 'O' { line_no as u64 } else { 0 })
        .sum()
}

fn sort_pattern(data: &mut Array2<char>) {
    data.columns_mut()
        .into_iter()
        .for_each(|mut col| sort_line(&mut col));
}

fn compute_load(data: &Array2<char>) -> u64 {
    data.columns().into_iter().map(compute_weight).sum()
}

fn task_1(input_file: &str) -> u64 {
    let data = read_to_string(input_file).unwrap();
    let mut pattern = Array2::parse_pattern(&data);

    sort_pattern(&mut pattern);
    compute_load(&mut pattern)
}

fn task_2(input_file: &str) -> u64 {
    let data = read_to_string(input_file).unwrap();
    let mut pattern = Array2::parse_pattern(&data);
    let mut cache: HashSet<Array2<char>> = HashSet::new();
    let mut loads = Vec::new();

    let n_cycles = 1_000_000_000;

    let mut i = 0;
    let mut start_i: Option<usize> = None;
    let mut first_repeated_pattern: Option<Array2<char>> = None;
    let mut end_i: Option<usize> = None;

    while end_i == None {
        for _ in 0..4 {
            sort_pattern(&mut pattern);
            pattern.rot90();
        }
        loads.push(compute_load(&pattern));
        if !cache.contains(&pattern) {
            cache.insert(pattern.clone());
        } else {
            if first_repeated_pattern == None {
                first_repeated_pattern = Some(pattern.clone());
                start_i = Some(i);
            } else {
                if end_i == None && first_repeated_pattern.clone().unwrap() == pattern {
                    end_i = Some(i);
                }
            }
        }
        i += 1;
    }
    let index = (n_cycles - start_i.unwrap()) % (end_i.unwrap() - start_i.unwrap()) + start_i.unwrap() - 1;
    loads[index]
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
    use ndarray::{array, Array1};

    #[test]
    fn test_transform_line() {
        let mut data = Array1::from_iter("OO.O.O..##".chars());
        sort_line(&mut data.view_mut());
        assert_eq!("OOOO....##", data.iter().collect::<String>());

        let mut data = Array1::from_iter("...OO....O".chars());
        sort_line(&mut data.view_mut());
        assert_eq!("OOO.......", data.iter().collect::<String>());

        let mut data = Array1::from_iter(".O...#O..O".chars());
        sort_line(&mut data.view_mut());
        assert_eq!("O....#OO..", data.iter().collect::<String>());

        let mut data = Array1::from_iter("..#...O.#.".chars());
        sort_line(&mut data.view_mut());
        assert_eq!("..#O....#.", data.iter().collect::<String>());
    }

    #[test]
    fn test_compute_load() {
        let mut data = Array2::parse_pattern(&vec![
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ].join("\n"));

        sort_pattern(&mut data);
        assert_eq!(136, compute_load(&mut data));
    }

    #[test]
    fn test_rot90() {
        let mut x = array![['1', '2'], ['3', '4'], ['5', '6']];
        x.rot90();
        let exp_x = array![['5', '3', '1'], ['6', '4', '2']];
        assert_eq!(exp_x, x);
    }

    #[test]
    fn test_task_1() {
        assert_eq!(136, task_1("./inputs/input_test_1.txt"))
    }

    #[test]
    fn test_task_2() {
        assert_eq!(64, task_2("./inputs/input_test_1.txt"))
    }
}
