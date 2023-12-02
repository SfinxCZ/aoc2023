use phf::phf_map;
use regex::Regex;

const DIGIT_MAP: phf::Map<&'static str, u32> = phf_map! {
    "0" => 0,
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
    "zero" => 0,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

pub(crate) struct Matcher {
    pattern_fwd: Regex,
    pattern_rev: Regex,
}

impl Matcher {
    fn from_characters(characters: Vec<&str>) -> Self {
        let pattern_fwd = characters.join("|");
        let pattern_rev = characters
            .iter()
            .map(|&s| s.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join("|");
        Matcher {
            pattern_fwd: Regex::new(&pattern_fwd).unwrap(),
            pattern_rev: Regex::new(&pattern_rev).unwrap(),
        }
    }

    pub(crate) fn default() -> Self {
        Matcher::from_characters(DIGIT_MAP.keys().map(|&s| s).collect())
    }

    fn find_digits(&self, line: &str) -> (String, String) {
        let reverse_line: String = line.to_owned().chars().rev().collect();
        let reverse_line_str = reverse_line.as_str();
        let first_digit = self.pattern_fwd.find(line).unwrap().as_str();
        let second_digit = self.pattern_rev.find(reverse_line_str).unwrap().as_str();
        (first_digit.to_owned(), second_digit.chars().rev().collect())
    }

    fn convert_digits(first_digit_str: &str, second_digit_str: &str) -> u32 {
        let first_digit = DIGIT_MAP.get(first_digit_str).unwrap();
        let second_digit = DIGIT_MAP.get(second_digit_str).unwrap();

        10 * first_digit + second_digit
    }

    pub fn find_coordinates(&self, line: &str) -> u32 {
        let (first_digit_str, second_digit_str) = self.find_digits(line);
        Matcher::convert_digits(&first_digit_str, &second_digit_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_characters() {
        let matcher = Matcher::from_characters(vec!["1", "2", "3", "one", "two", "three"]);
        assert_eq!(matcher.pattern_fwd.as_str(), "1|2|3|one|two|three");
        assert_eq!(matcher.pattern_rev.as_str(), "1|2|3|eno|owt|eerht");
    }

    #[test]
    fn test_convert_digits_to_number() {
        let number = Matcher::convert_digits("one", "5");
        assert_eq!(number, 15)
    }

    #[test]
    fn test_find_calibration_values_multiple_digits_in_string() {
        let matcher = Matcher::default();
        let coordinates = matcher.find_coordinates("25144spnmfvvj1dxpqmhsldk");
        assert_eq!(coordinates, 21)
    }

    #[test]
    fn test_find_calibration_values() {
        let matcher = Matcher::default();
        let coordinates = matcher.find_coordinates("1abc2");
        assert_eq!(coordinates, 12);
    }

    #[test]
    fn test_find_calibration_values_example_1() {
        let matcher = Matcher::default();
        let coordinates = matcher.find_coordinates("8825eightknfv");
        assert_eq!(coordinates, 88);
    }

    #[test]
    fn test_find_calibration_values_single_digit_in_string() {
        let matcher = Matcher::default();
        let coordinates = matcher.find_coordinates("blaabalal1lalksdlkjas");
        assert_eq!(coordinates, 11);
    }


    #[test]
    fn test_find_calibration_values_single_digit_and_digit_name_in_string() {
        let matcher = Matcher::default();
        let coordinates = matcher.find_coordinates("nineninebsbd8");
        assert_eq!(coordinates, 98);
    }
}
