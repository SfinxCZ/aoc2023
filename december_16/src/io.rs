use ndarray::Array2;

pub fn parse_pattern(data: &str) -> Array2<char> {
    let chars = data
        .trim()
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let nrows = chars.len();
    let ncols = chars[0].len();

    Array2::from_shape_vec((nrows, ncols), chars.iter().flatten().map(|&c| c).collect()).unwrap()
}