use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Clone, Copy)]
enum Card {
    Card2,
    Card3,
    Card4,
    Card5,
    Card6,
    Card7,
    Card8,
    Card9,
    CardT,
    CardJ,
    CardQ,
    CardK,
    CardA,
}

impl Card {
    pub fn from(input: char) -> Self {
        return match input {
            'A' => Card::CardA,
            'K' => Card::CardK,
            'Q' => Card::CardQ,
            'J' => Card::CardJ,
            'T' => Card::CardT,
            '9' => Card::Card9,
            '8' => Card::Card8,
            '7' => Card::Card7,
            '6' => Card::Card6,
            '5' => Card::Card5,
            '4' => Card::Card4,
            '3' => Card::Card3,
            '2' => Card::Card2,
            _ => {
                panic!("Unknown input {}", input)
            }
        };
    }

    pub fn from_str(input: &str) -> Vec<Self> {
        input
            .chars()
            .into_iter()
            .map(Card::from)
            .collect::<Vec<Card>>()
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    HighCard(Card, Card, Card, Card, Card),
    OnePair(Card, Card, Card, Card, Card),
    TwoPairs(Card, Card, Card, Card, Card),
    ThreeOfAKind(Card, Card, Card, Card, Card),
    FullHouse(Card, Card, Card, Card, Card),
    FourOfAKind(Card, Card, Card, Card, Card),
    FiveOfAKind(Card, Card, Card, Card, Card),
}

impl HandType {
    fn card_counts(input: &[Card]) -> HashMap<&Card, u8> {
        let mut mapping: HashMap<&Card, u8> = HashMap::new();
        for card in input {
            let count = mapping.entry(card).or_insert(0);
            *count += 1;
        }
        mapping
    }

    pub fn from(input: &[Card]) -> Self {
        let mapping = HandType::card_counts(input);
        let mut counts = mapping.values().collect::<Vec<&u8>>();
        counts.sort();
        counts.reverse();

        let t: (Card, Card, Card, Card, Card) = (
            *input.get(0).unwrap(),
            *input.get(1).unwrap(),
            *input.get(2).unwrap(),
            *input.get(3).unwrap(),
            *input.get(4).unwrap(),
        );

        match &counts[..] {
            [5, ..] => HandType::FiveOfAKind(t.0, t.1, t.2, t.3, t.4),
            [4, ..] => HandType::FourOfAKind(t.0, t.1, t.2, t.3, t.4),
            [3, 2, ..] => HandType::FullHouse(t.0, t.1, t.2, t.3, t.4),
            [3, 1, 1, ..] => HandType::ThreeOfAKind(t.0, t.1, t.2, t.3, t.4),
            [2, 2, 1, ..] => HandType::TwoPairs(t.0, t.1, t.2, t.3, t.4),
            [2, 1, 1, 1, ..] => HandType::OnePair(t.0, t.1, t.2, t.3, t.4),
            [1, 1, 1, 1, 1, ..] => HandType::HighCard(t.0, t.1, t.2, t.3, t.4),
            _ => panic!("Unknown hand {:?}", counts),
        }
    }

    fn from_str(input: &str) -> Self {
        HandType::from(&Card::from_str(input))
    }
}

fn parse_line(line: &str) -> (HandType, u32) {
    let (cards_str, bid_str) = line.split_once(" ").unwrap();
    (HandType::from_str(cards_str), bid_str.parse().unwrap())
}

pub(crate) fn run(input_file: &str) -> u32 {
    let mut hands = read_to_string(input_file)
        .unwrap()
        .lines()
        .map(parse_line)
        .collect::<Vec<(HandType, u32)>>();
    hands.sort_by(|(hand_1, _), (hand_2, _)| hand_1.cmp(hand_2));
    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| ((rank as u32) + 1) * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cards() {
        assert_eq!(
            Card::from_str("32T3K"),
            vec![
                Card::Card3,
                Card::Card2,
                Card::CardT,
                Card::Card3,
                Card::CardK
            ]
        );
        assert_eq!(
            Card::from_str("T55J5"),
            vec![
                Card::CardT,
                Card::Card5,
                Card::Card5,
                Card::CardJ,
                Card::Card5
            ]
        );
    }

    #[test]
    fn test_parse_hands() {
        assert_eq!(
            HandType::from(&Card::from_str("AAAAA")),
            HandType::FiveOfAKind(
                Card::CardA,
                Card::CardA,
                Card::CardA,
                Card::CardA,
                Card::CardA
            )
        );
        assert_eq!(
            HandType::from(&Card::from_str("AA8AA")),
            HandType::FourOfAKind(
                Card::CardA,
                Card::CardA,
                Card::Card8,
                Card::CardA,
                Card::CardA
            )
        );
        assert_eq!(
            HandType::from(&Card::from_str("23332")),
            HandType::FullHouse(
                Card::Card2,
                Card::Card3,
                Card::Card3,
                Card::Card3,
                Card::Card2
            )
        );
        assert_eq!(
            HandType::from(&Card::from_str("TTT98")),
            HandType::ThreeOfAKind(
                Card::CardT,
                Card::CardT,
                Card::CardT,
                Card::Card9,
                Card::Card8
            )
        );
        assert_eq!(
            HandType::from(&Card::from_str("23432")),
            HandType::TwoPairs(
                Card::Card2,
                Card::Card3,
                Card::Card4,
                Card::Card3,
                Card::Card2
            )
        );
        assert_eq!(
            HandType::from(&Card::from_str("A23A4")),
            HandType::OnePair(
                Card::CardA,
                Card::Card2,
                Card::Card3,
                Card::CardA,
                Card::Card4
            )
        );
        assert_eq!(
            HandType::from(&Card::from_str("23456")),
            HandType::HighCard(
                Card::Card2,
                Card::Card3,
                Card::Card4,
                Card::Card5,
                Card::Card6
            )
        );
    }

    #[test]
    fn test_comparator_different_hand_type_with_same_card() {
        let c = Card::Card2;
        assert!(HandType::HighCard(c, c, c, c, c) < HandType::OnePair(c, c, c, c, c));
        assert!(HandType::OnePair(c, c, c, c, c) < HandType::TwoPairs(c, c, c, c, c));
        assert!(HandType::TwoPairs(c, c, c, c, c) < HandType::ThreeOfAKind(c, c, c, c, c));
        assert!(HandType::ThreeOfAKind(c, c, c, c, c) < HandType::FullHouse(c, c, c, c, c));
        assert!(HandType::FullHouse(c, c, c, c, c) < HandType::FourOfAKind(c, c, c, c, c));
        assert!(HandType::FourOfAKind(c, c, c, c, c) < HandType::FiveOfAKind(c, c, c, c, c));
    }

    #[test]
    fn test_comparator_same_hand_type_with_different_cards() {
        assert!(HandType::from_str("33332") > HandType::from_str("2AAAA"));
    }

    #[test]
    fn test_comparator_card() {
        assert!(Card::Card2 < Card::Card3);
        assert!(Card::Card3 < Card::Card4);
        assert!(Card::Card4 < Card::Card5);
        assert!(Card::Card5 < Card::Card6);
        assert!(Card::Card6 < Card::Card7);
        assert!(Card::Card7 < Card::Card8);
        assert!(Card::Card8 < Card::Card9);
        assert!(Card::Card9 < Card::CardT);
        assert!(Card::CardT < Card::CardJ);
        assert!(Card::CardJ < Card::CardQ);
        assert!(Card::CardQ < Card::CardK);
        assert!(Card::CardK < Card::CardA);
    }

    #[test]
    fn test_task_1() {
        assert_eq!(run("./inputs/input_test.txt"), 6440);
    }
}
