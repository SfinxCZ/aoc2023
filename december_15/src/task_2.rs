use lazy_static::lazy_static;
use regex::Regex;
use crate::hashmap::CustomHash;

impl CustomHash for String {
    fn compute_hash(&self) -> usize {
        let mut current_value: usize = 0;
        self.chars().for_each(|ch| {
            current_value += ch as usize;
            current_value *= 17;
            current_value = current_value % 256;
        });
        current_value
    }
}


#[derive(PartialEq, Debug)]
pub enum Operation {
    Insert(u64),
    Remove
}

lazy_static! {
     static ref PATTERN: Regex = Regex::new("^(.+)(((=)([0-9]+))|(-))$").unwrap();
}

pub fn parse_element(input: &str) -> (String, Operation) {
    let captures = PATTERN.captures(input).unwrap();
    match &captures[2].chars().nth(0) {
        Some('=') => (captures[1].to_owned(), Operation::Insert(captures[5].parse().unwrap())),
        Some('-') => (captures[1].to_owned(), Operation::Remove),
        operation => panic!("Unknown operation {:?}", operation)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_element() {
        assert_eq!(("qp".to_string(), Operation::Insert(3)), parse_element("qp=3"));
        assert_eq!(("cm".to_string(), Operation::Remove), parse_element("cm-"));
    }
}