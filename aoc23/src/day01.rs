use std::collections::HashMap;

pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let mut trie = Trie::new(&[]);
    trie.insert("1", 1);
    trie.insert("2", 2);
    trie.insert("3", 3);
    trie.insert("4", 4);
    trie.insert("5", 5);
    trie.insert("6", 6);
    trie.insert("7", 7);
    trie.insert("8", 8);
    trie.insert("9", 9);

    let mut result: u32 = 0;
    for line in input.lines() {
        result += get_number(line, &trie, &trie);
    }

    Ok(result.to_string())
}

fn part2(input: &str) -> Result<String, &'static str> {
    let mut forward_trie = Trie::new(&[]);
    forward_trie.insert("1", 1);
    forward_trie.insert("one", 1);
    forward_trie.insert("2", 2);
    forward_trie.insert("two", 2);
    forward_trie.insert("3", 3);
    forward_trie.insert("three", 3);
    forward_trie.insert("4", 4);
    forward_trie.insert("four", 4);
    forward_trie.insert("5", 5);
    forward_trie.insert("five", 5);
    forward_trie.insert("6", 6);
    forward_trie.insert("six", 6);
    forward_trie.insert("7", 7);
    forward_trie.insert("seven", 7);
    forward_trie.insert("8", 8);
    forward_trie.insert("eight", 8);
    forward_trie.insert("9", 9);
    forward_trie.insert("nine", 9);

    let mut backward_trie = Trie::new(&[]);
    backward_trie.insert("1", 1);
    backward_trie.insert("eno", 1);
    backward_trie.insert("2", 2);
    backward_trie.insert("owt", 2);
    backward_trie.insert("3", 3);
    backward_trie.insert("eerht", 3);
    backward_trie.insert("4", 4);
    backward_trie.insert("ruof", 4);
    backward_trie.insert("5", 5);
    backward_trie.insert("evif", 5);
    backward_trie.insert("6", 6);
    backward_trie.insert("xis", 6);
    backward_trie.insert("7", 7);
    backward_trie.insert("neves", 7);
    backward_trie.insert("8", 8);
    backward_trie.insert("thgie", 8);
    backward_trie.insert("9", 9);
    backward_trie.insert("enin", 9);

    let mut result: u32 = 0;
    for line in input.lines() {
        result += get_number(line, &forward_trie, &backward_trie);
    }

    Ok(result.to_string())
}

/// get_number takes an input string and both a forward and backward [Trie], returning
/// a u32 representation of the concatentation of the first digit found in the string using
/// the forward trie and the last digit found in the backward trie.
fn get_number(input: &str, forward_trie: &Trie, backward_trie: &Trie) -> u32 {
    let mut result = 0;

    let first_digit = get_first_match(input, forward_trie);
    match first_digit {
        Some(val) => {
            result += val * 10;
        }
        None => panic!("no match!"),
    }
    let reversed_string: String = input.chars().rev().collect();
    let second_digit = get_first_match(&reversed_string[..], backward_trie);
    match second_digit {
        Some(val) => {
            result += val;
        }
        None => panic!("no match!"),
    }

    result
}

fn get_first_match(word: &str, trie: &Trie) -> Option<u32> {
    if word.is_empty() {
        return None;
    }
    match trie.search(word) {
        Some(val) => Some(val),
        None => get_first_match(&word[1..], trie),
    }
}

struct Trie {
    value: Option<u32>,
    children: HashMap<char, Trie>,
}

struct TrieNode<'a> {
    word: &'a str,
    value: u32,
}

impl Trie {
    fn new(nodes: &[TrieNode]) -> Trie {
        let mut root = Trie {
            value: None,
            children: HashMap::new(),
        };

        for node in nodes {
            root.insert(node.word, node.value);
        }

        root
    }

    fn insert(&mut self, input: &str, value: u32) {
        // There's no input left, so this node is where we place the value
        if input.is_empty() {
            self.value = Some(value);
            return;
        }

        let mut chars = input.chars();
        // this is the next letter in the string (assuming ASCII)
        match chars.next() {
            Some(c) => {
                self.children.entry(c).or_insert(Trie {
                    value: None,
                    children: HashMap::new(),
                });
                match self.children.get_mut(&c) {
                    Some(t) => t.insert(chars.as_str(), value),
                    None => panic!("a child entry should be here but isn't"),
                }
            }
            None => panic!("chars.next() empty while input length wasn't 0"),
        }
    }

    fn search(&self, input: &str) -> Option<u32> {
        if let Some(val) = self.value {
            return Some(val);
        }
        if input.is_empty() {
            return self.value;
        }

        let mut chars = input.chars();
        match chars.next() {
            Some(c) => match self.children.get(&c) {
                Some(t) => t.search(chars.as_str()),
                None => None,
            },
            None => panic!("chars.next() empty while input length wasn't 0"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trie() {
        let mut t = Trie::new(&[
            TrieNode {
                word: "hello",
                value: 5,
            },
            TrieNode {
                word: "hola",
                value: 10,
            },
        ]);
        t.insert("8", 8);
        {
            let result = t.search("hello");
            assert_eq!(result, Some(5));
        }
        {
            let result = t.search("hola");
            assert_eq!(result, Some(10));
        }
        {
            let result = t.search("8");
            assert_eq!(result, Some(8));
        }
    }

    #[test]
    fn test_get_number_part_1() {
        let mut trie = Trie::new(&[]);
        trie.insert("1", 1);
        trie.insert("2", 2);
        trie.insert("3", 3);
        trie.insert("4", 4);
        trie.insert("5", 5);
        trie.insert("6", 6);
        trie.insert("7", 7);
        trie.insert("8", 8);
        trie.insert("9", 9);

        {
            let result = get_number("1abc2", &trie, &trie);
            assert_eq!(result, 12);
        }
        {
            let result = get_number("pqr3stu8vwx", &trie, &trie);
            assert_eq!(result, 38);
        }
        {
            let result = get_number("a1b2c3d4e5f", &trie, &trie);
            assert_eq!(result, 15);
        }
        {
            let result = get_number("treb7uchet", &trie, &trie);
            assert_eq!(result, 77);
        }
    }

    #[test]
    fn test_get_number_part_2() {
        let mut forward_trie = Trie::new(&[]);
        forward_trie.insert("1", 1);
        forward_trie.insert("one", 1);
        forward_trie.insert("2", 2);
        forward_trie.insert("two", 2);
        forward_trie.insert("3", 3);
        forward_trie.insert("three", 3);
        forward_trie.insert("4", 4);
        forward_trie.insert("four", 4);
        forward_trie.insert("5", 5);
        forward_trie.insert("five", 5);
        forward_trie.insert("6", 6);
        forward_trie.insert("six", 6);
        forward_trie.insert("7", 7);
        forward_trie.insert("seven", 7);
        forward_trie.insert("8", 8);
        forward_trie.insert("eight", 8);
        forward_trie.insert("9", 9);
        forward_trie.insert("nine", 9);

        let mut backward_trie = Trie::new(&[]);
        backward_trie.insert("1", 1);
        backward_trie.insert("eno", 1);
        backward_trie.insert("2", 2);
        backward_trie.insert("owt", 2);
        backward_trie.insert("3", 3);
        backward_trie.insert("eerht", 3);
        backward_trie.insert("4", 4);
        backward_trie.insert("ruof", 4);
        backward_trie.insert("5", 5);
        backward_trie.insert("evif", 5);
        backward_trie.insert("6", 6);
        backward_trie.insert("xis", 6);
        backward_trie.insert("7", 7);
        backward_trie.insert("neves", 7);
        backward_trie.insert("8", 8);
        backward_trie.insert("thgie", 8);
        backward_trie.insert("9", 9);
        backward_trie.insert("enin", 9);
        {
            let input = "two1nine";
            let result = get_number(input, &forward_trie, &backward_trie);
            assert_eq!(result, 29);
        }
        {
            let input = "eightwothree";
            let result = get_number(input, &forward_trie, &backward_trie);
            assert_eq!(result, 83);
        }
        {
            let input = "abcone2threexyz";
            let result = get_number(input, &forward_trie, &backward_trie);
            assert_eq!(result, 13);
        }
        {
            let input = "xtwone3four";
            let result = get_number(input, &forward_trie, &backward_trie);
            assert_eq!(result, 24);
        }
        {
            let input = "4nineeightseven2";
            let result = get_number(input, &forward_trie, &backward_trie);
            assert_eq!(result, 42);
        }
        {
            let input = "zoneight234";
            let result = get_number(input, &forward_trie, &backward_trie);
            assert_eq!(result, 14);
        }
        {
            let input = "7pqrstsixteen";
            let result = get_number(input, &forward_trie, &backward_trie);
            assert_eq!(result, 76);
        }
    }
}
