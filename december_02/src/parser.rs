use std::cmp::max;

pub struct Game {
    game_id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    pub fn game_id(&self) -> u32 {
        self.game_id
    }
    pub fn parse_game(line: &str) -> Self {
        let (game_name, definition) = line.split_once(":").unwrap();
        let game_id: u32 = game_name.trim().split_once(" ").unwrap().1.parse().unwrap();
        let game_sample_max: GameSample = definition
            .split(";")
            .map(GameSample::from_str)
            .fold(GameSample::default(), GameSample::max);
        Game {
            game_id,
            red: game_sample_max.red,
            green: game_sample_max.green,
            blue: game_sample_max.blue,
        }
    }

    pub fn is_valid(&self, game_sample: &GameSample) -> bool {
        self.red <= game_sample.red
            && self.green <= game_sample.green
            && self.blue <= game_sample.blue
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Default)]
pub struct GameSample {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameSample {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        GameSample { red, green, blue }
    }
    fn from_str(sample_str: &str) -> Self {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        sample_str.split(",").for_each(|chunk| {
            let parts = chunk.trim().split_once(" ").unwrap();
            match parts {
                (c_red, "red") => red = c_red.trim().parse().unwrap(),
                (c_green, "green") => green = c_green.trim().parse().unwrap(),
                (c_blue, "blue") => blue = c_blue.trim().parse().unwrap(),
                _ => {
                    panic!("Unknown color")
                }
            }
        });
        GameSample { red, green, blue }
    }

    fn max(lhs_game: GameSample, rhs_game: GameSample) -> GameSample {
        GameSample::new(
            max(lhs_game.red, rhs_game.red),
            max(lhs_game.green, rhs_game.green),
            max(lhs_game.blue, rhs_game.blue),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_green_missing() {
        let game_sample = GameSample::from_str("3 blue, 4 red");
        assert_eq!(game_sample.blue, 3);
        assert_eq!(game_sample.red, 4);
        assert_eq!(game_sample.green, 0);
    }

    #[test]
    fn test_from_str() {
        let game_sample = GameSample::from_str("3 blue, 4 red, 12 green");
        assert_eq!(game_sample.blue, 3);
        assert_eq!(game_sample.red, 4);
        assert_eq!(game_sample.green, 12);
    }

    #[test]
    fn test_add() {
        let sample1 = GameSample {
            red: 1,
            green: 20,
            blue: 3,
        };
        let sample2 = GameSample {
            red: 10,
            green: 2,
            blue: 30,
        };

        let sample_max = GameSample::max(sample1, sample2);
        assert_eq!(sample_max.red, 10);
        assert_eq!(sample_max.green, 20);
        assert_eq!(sample_max.blue, 30);
    }

    #[test]
    fn test_parse_game() {
        let game = Game::parse_game(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );
        assert_eq!(game.game_id, 3);
        assert_eq!(game.red, 20);
        assert_eq!(game.green, 13);
        assert_eq!(game.blue, 6);
    }

    #[test]
    fn test_is_valid() {
        let game = Game {
            game_id: 1,
            red: 10,
            green: 12,
            blue: 15,
        };
        assert!(game.is_valid(&GameSample {
            red: 30,
            green: 50,
            blue: 50
        }));
        assert!(!game.is_valid(&GameSample {
            red: 3,
            green: 50,
            blue: 50
        }));
        assert!(!game.is_valid(&GameSample {
            red: 30,
            green: 5,
            blue: 50
        }));
        assert!(!game.is_valid(&GameSample {
            red: 30,
            green: 50,
            blue: 5
        }));
    }

    #[test]
    fn test_power() {
        let game_1 = Game::parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let game_2 = Game::parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        let game_3 = Game::parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        let game_4 = Game::parse_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        let game_5 = Game::parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

        assert_eq!(game_1.power(), 48);
        assert_eq!(game_2.power(), 12);
        assert_eq!(game_3.power(), 1560);
        assert_eq!(game_4.power(), 630);
        assert_eq!(game_5.power(), 36);
    }
}
