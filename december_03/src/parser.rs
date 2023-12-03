use lazy_static::lazy_static;
use regex::{Match, Regex};

lazy_static! {
    pub static ref NUMBER_PATTERN: Regex = Regex::new("([0-9]+)").unwrap();
    pub static ref SYMBOL_PATTERN: Regex = Regex::new("([^0-9.])").unwrap();
    pub static ref GEAR_PATTERN: Regex = Regex::new("(\\*)").unwrap();
}

#[derive(PartialEq, Debug)]
pub struct Number {
    number: u32,
    line_no: usize,
    start: usize,
    end: usize,
}

impl From<(Match<'_>, usize)> for Number {
    fn from(match_and_line_no: (Match, usize)) -> Self {
        let (m, line_no) = match_and_line_no;
        Number {
            line_no,
            number: m.as_str().trim().parse().unwrap(),
            start: m.start(),
            end: m.end() - 1,
        }
    }
}

impl Number {
    pub fn is_close_to(&self, line: usize, position: usize) -> bool {
        let pos_start = if self.start > 1 { self.start - 1 } else { 0 };
        let pos_end = self.end + 1;
        let line_start = if self.line_no > 1 { self.line_no - 1 } else { 0 };
        let line_end = self.line_no + 1;
        (pos_start <= position) && (position <= pos_end) && (line_start <= line) && (line <= line_end)
    }

    pub fn number(&self) -> u32 {
        self.number
    }
}

#[derive(PartialEq, Debug)]
pub struct Symbol {
    symbol: String,
    line_no: usize,
    position: usize,
}

impl From<(Match<'_>, usize)> for Symbol {
    fn from(match_and_line_no: (Match, usize)) -> Self {
        let (m, line_no) = match_and_line_no;
        Symbol {
            line_no,
            symbol: m.as_str().to_owned(),
            position: m.start(),
        }
    }
}

impl Symbol {
    pub fn line_no(&self) -> usize {
        self.line_no
    }
    pub fn position(&self) -> usize {
        self.position
    }
}

pub fn parse_line<T>(line: &str, line_no: usize, pattern: &Regex) -> Vec<T>
where
    T: for<'a> From<(Match<'a>, usize)>,
{
    let captures = pattern.captures_iter(line);
    let numbers = captures
        .filter_map(|m| m.get(0))
        .map(|m| T::from((m, line_no)))
        .collect::<Vec<T>>();
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let numbers: Vec<Number> = parse_line("467..114..", 0, &NUMBER_PATTERN);
        assert_eq!(
            numbers,
            vec![
                Number {
                    line_no: 0,
                    number: 467,
                    start: 0,
                    end: 2
                },
                Number {
                    line_no: 0,
                    number: 114,
                    start: 5,
                    end: 7
                },
            ]
        );
        let symbols: Vec<Symbol> = parse_line("467..114..", 0, &SYMBOL_PATTERN);
        assert_eq!(symbols, Vec::new());

        let numbers: Vec<Number> = parse_line("......#...", 1, &NUMBER_PATTERN);
        assert_eq!(numbers, Vec::new());
        let symbols: Vec<Symbol> = parse_line("......#...", 1, &SYMBOL_PATTERN);
        assert_eq!(
            symbols,
            vec![Symbol {
                symbol: "#".to_string(),
                line_no: 1,
                position: 6
            }]
        );

        let numbers: Vec<Number> = parse_line(".....+.58.§", 2, &NUMBER_PATTERN);
        assert_eq!(
            numbers,
            vec![Number {
                line_no: 2,
                number: 58,
                start: 7,
                end: 8
            }]
        );
        let symbols: Vec<Symbol> = parse_line(".....+.58.§", 2, &SYMBOL_PATTERN);
        assert_eq!(
            symbols,
            vec![
                Symbol {
                    line_no: 2,
                    symbol: "+".to_string(),
                    position: 5
                },
                Symbol {
                    line_no: 2,
                    symbol: "§".to_string(),
                    position: 10
                }
            ]
        );

        // let (numbers, symbols) = parse_line(".....+.58.");
    }

    #[test]
    fn test_parse_line_gears(){
        let symbols: Vec<Symbol> = parse_line("467..114..", 0, &GEAR_PATTERN);
        assert_eq!(symbols, Vec::new());

        let symbols: Vec<Symbol> = parse_line(".....*....", 0, &GEAR_PATTERN);
        assert_eq!(symbols, vec![Symbol{symbol: "*".to_string(), line_no: 0, position: 5}]);

        let symbols: Vec<Symbol> = parse_line("617*......", 0, &GEAR_PATTERN);
        assert_eq!(symbols, vec![Symbol {symbol: "*".to_string(), line_no: 0, position: 3}]);
    }

    #[test]
    fn test_is_close_to() {
        let number = Number {
            line_no: 4,
            number: 467,
            start: 3,
            end: 5,
        };

        for row in 0..10 {
            for col in 0..10 {
                let exp_is_close = (3 <= row) && (row <= 5) && (2 <= col) && (col <= 6);
                assert_eq!(
                    number.is_close_to(row, col),
                    exp_is_close,
                    "row: {}, col {}",
                    row,
                    col
                );
            }
        }
    }

    #[test]
    fn test_is_close_to_on_egde() {
        let number = Number {
            line_no: 1,
            number: 467,
            start: 1,
            end: 3,
        };

        for row in 0..10 {
            for col in 0..10 {
                let exp_is_close = (0 <= row) && (row <= 2) && (0 <= col) && (col <= 4);
                assert_eq!(
                    number.is_close_to(row, col),
                    exp_is_close,
                    "row: {}, col {}",
                    row,
                    col
                );
            }
        }
    }

    #[test]
    fn test_is_close_to_on_egde_2() {
        let number = Number {
            line_no: 0,
            number: 467,
            start: 0,
            end: 2,
        };

        for row in 0..10 {
            for col in 0..10 {
                let exp_is_close = (0 <= row) && (row <= 1) && (0 <= col) && (col <= 3);
                assert_eq!(
                    number.is_close_to(row, col),
                    exp_is_close,
                    "row: {}, col {}",
                    row,
                    col
                );
            }
        }
    }
}
