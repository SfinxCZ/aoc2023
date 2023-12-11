use std::fs::read_to_string;

#[derive(Eq, PartialEq, Debug)]
pub struct Galaxy {
    id: i64,
    row: usize,
    col: usize,
}

impl Galaxy {

    pub fn distance(&self, other: &Galaxy) -> u64 {
        let row_dist = ((self.row as i64) - (other.row as i64)).abs();
        let col_dist = ((self.col as i64) - (other.col as i64)).abs();
        (row_dist + col_dist) as u64
    }
}

pub fn find_expansion_offsets_rows(map: &Vec<Vec<char>>, multiplier: usize) -> Vec<usize> {
    let mut expansions: Vec<usize> = Vec::new();
    let mut i = 0;
    for row in map.iter() {
        expansions.push(i);
        if !row.contains(&'#') {
            i += multiplier;
        } else {
            i += 1;
        }
    }
    expansions
}

pub fn find_expansion_offsets_columns(map: &Vec<Vec<char>>, multiplier: usize) -> Vec<usize> {
    let mut expansions: Vec<usize> = Vec::new();
    let nrows = map.len();
    let ncols = map[0].len();

    let mut i = 0;
    for col in 0..ncols {
        let mut contains_galaxy = false;
        for row in 0..nrows {
            if map[row][col] == '#' {
                contains_galaxy = true;
                break;
            }
        }
        expansions.push(i);
        if !contains_galaxy {
            i += multiplier;
        } else {
            i += 1;
        }
    }
    expansions
}

pub fn parse_file(input_file: &str, multiplier: usize) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = Vec::new();

    let data: Vec<Vec<char>> = read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let expansions_columns = find_expansion_offsets_columns(&data, multiplier);
    let expansions_rows = find_expansion_offsets_rows(&data, multiplier);

    for (line, row) in data.iter().zip(expansions_rows) {
        for (char, col) in line.iter().zip(expansions_columns.clone()) {
            if *char == '#' {
                galaxies.push(Galaxy {
                    id: galaxies.len() as i64 + 1,
                    row,
                    col,
                })
            }
        }
    }
    galaxies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_expansion_rows() {
        let map = vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '#', '.', '.'],
            vec!['.', '.', '#', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '#', '.', '.'],
        ];
        let offsets = find_expansion_offsets_rows(&map, 5);
        assert_eq!(vec![0, 5, 6, 7, 12], offsets);
    }

    #[test]
    fn test_find_expansion_cols() {
        let map = vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '#', '.', '.'],
            vec!['.', '.', '#', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '#', '.', '.'],
        ];
        let offsets = find_expansion_offsets_columns(&map, 5);
        assert_eq!(vec![0, 5, 6, 7], offsets);
    }

    #[test]
    fn test_parse_file() {
        let galaxies = parse_file("inputs/input_test_1.txt", 2);
        let exp_galaxies = vec![
            Galaxy {
                id: 1,
                row: 0,
                col: 4,
            },
            Galaxy {
                id: 2,
                row: 1,
                col: 9,
            },
            Galaxy {
                id: 3,
                row: 2,
                col: 0,
            },
            Galaxy {
                id: 4,
                row: 5,
                col: 8,
            },
            Galaxy {
                id: 5,
                row: 6,
                col: 1,
            },
            Galaxy {
                id: 6,
                row: 7,
                col: 12,
            },
            Galaxy {
                id: 7,
                row: 10,
                col: 9,
            },
            Galaxy {
                id: 8,
                row: 11,
                col: 0,
            },
            Galaxy {
                id: 9,
                row: 11,
                col: 5,
            },
        ];
        assert_eq!(galaxies, exp_galaxies);
    }
}
