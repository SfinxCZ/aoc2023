use std::fs::read_to_string;
use crate::parser::{Game, GameSample};

mod parser;

fn task_1() {
    let game_sample = GameSample::new(12, 13, 14);

    let sum_of_games: u32 = read_to_string("./input/input_part1.txt")
            .unwrap()
            .lines()
            .map(Game::parse_game)
        .filter(|game| game.is_valid(&game_sample))
        .map(|game| game.game_id())
        .sum();
    println!("{}", sum_of_games)
}


fn task_2() {
    let sum_of_games: u32 = read_to_string("./input/input_part1.txt")
        .unwrap()
        .lines()
        .map(Game::parse_game)
        .map(|game| game.power())
        .sum();
    println!("{}", sum_of_games)
}


fn main() {
    task_2();
}