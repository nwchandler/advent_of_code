use std::collections::HashMap;

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let mut result: u32 = 0;

    let mut categories: Vec<Vec<u32>> =
        vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    let mut hand_to_bid: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let mut line_iter = line.trim().split_whitespace();
        let hand = convert_hand(line_iter.next().unwrap());
        let bid = line_iter.next().unwrap().parse::<u32>().unwrap();
        let hand_numeric_value = u32::from_str_radix(hand.as_str(), 16).unwrap();

        let category = match categorize_hand(hand.as_str()) {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        };
        categories[category].push(hand_numeric_value);
        hand_to_bid.insert(hand_numeric_value, bid);
    }

    let mut place = 1;
    for mut category in categories {
        category.sort();
        for hand in category {
            result += place * hand_to_bid.get(&hand).unwrap();
            place += 1;
        }
    }

    Ok(result.to_string())
}

fn part2(input: &str) -> Result<String, &'static str> {
    let result: u32 = 0;

    Ok(result.to_string())
}

/// convert_hand takes a string formatted as a hand and converts to hex formatting for easier work
/// downstream.
fn convert_hand(input: &str) -> String {
    input
        .replace('A', "E")
        .replace('K', "D")
        .replace('Q', "C")
        .replace('J', "B")
        .replace('T', "A")
}

fn categorize_hand(input: &str) -> HandType {
    let trimmed_input = input.trim();
    let mut count_map: HashMap<char, u8> = HashMap::new();
    let mut count_vec: Vec<u8> = vec![0; 5];

    for (i, c) in trimmed_input.chars().enumerate() {
        let idx = count_map.entry(c).or_insert(i as u8).to_owned();
        count_vec[idx as usize] += 1;
    }

    count_vec.sort();
    match count_vec[4] {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => match count_vec[3] {
            2 => HandType::FullHouse,
            _ => HandType::ThreeOfAKind,
        },
        2 => match count_vec[3] {
            2 => HandType::TwoPair,
            _ => HandType::OnePair,
        },
        _ => HandType::HighCard,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_hand() {
        {
            let input = "T55J5";
            let result = convert_hand(input);
            assert_eq!(result, "A55B5");
        }
        {
            let input = "KK677";
            let result = convert_hand(input);
            assert_eq!(result, "DD677");
        }
        {
            let input = "KTJJT";
            let result = convert_hand(input);
            assert_eq!(result, "DABBA");
        }
        {
            let input = "QQQJA";
            let result = convert_hand(input);
            assert_eq!(result, "CCCBE");
        }
    }

    #[test]
    fn test_categorize_hand() {
        {
            // five of a kind
            let input = "AAAAA";
            let result = categorize_hand(input);
            assert!(matches!(result, HandType::FiveOfAKind));
        }
        {
            // four of a kind
            let input = "AA8AA";
            let result = categorize_hand(input);
            assert!(matches!(result, HandType::FourOfAKind));
        }
        {
            // full house
            let input = "23332";
            let result = categorize_hand(input);
            assert!(matches!(result, HandType::FullHouse));
        }
        {
            // three of a kind
            let input = "TTT98";
            let result = categorize_hand(input);
            assert!(matches!(result, HandType::ThreeOfAKind));
        }
        {
            // two pair
            let input = "23432";
            let result = categorize_hand(input);
            assert!(matches!(result, HandType::TwoPair));
        }
        {
            // one pair
            let input = "A23A4";
            let result = categorize_hand(input);
            assert!(matches!(result, HandType::OnePair));
        }
        {
            // high card
            let input = "23456";
            let result = categorize_hand(input);
            assert!(matches!(result, HandType::HighCard));
        }
    }

    #[test]
    fn integration_test_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = part1(input);
        assert_eq!(result.unwrap(), "6440");
    }

    // TODO: Don't ignore after implementing
    #[ignore]
    #[test]
    fn integration_test_part2() {
        let input = "";
        let result = part2(input);
        assert_eq!(result.unwrap(), "??");
    }
}
