pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let mut result: u32 = 0;

    for mut row in input.lines() {
        // Get rid of "Card xx: " prefix
        row = row.split(":").nth(1).unwrap();
        let mut row_sections = row.split("|");

        // extract winning numbers
        let winning_numbers = extract_numbers(row_sections.next().unwrap());

        // extract and sort actual numbers
        let mut actual_numbers = extract_numbers(row_sections.next().unwrap());
        actual_numbers.sort();

        let mut match_count: u32 = 0;

        for num in winning_numbers {
            if let Ok(_) = actual_numbers.binary_search(&num) {
                match_count += 1;
            }
        }

        // When there are > 0 matches, we bit shift 1 by the match, to get 2 * the expected total.
        // When there are 0 matches, we bit shift 1 by 0, to get 1.
        // In both cases, we then right-shift by 1 place, to either: cut the total (match > 0) in
        // half, or else shift the 1 out completely to get 0 (match == 0). In either case, the
        // total is the expected amount.
        result += (1 << match_count) >> 1;
    }

    Ok(result.to_string())
}

fn part2(input: &str) -> Result<String, &'static str> {
    let mut result: u32 = 0;

    let line_count = input.lines().count();
    // initialize a vector, with 1 card for each game, so that we don't run into issues with index
    // errors later on.
    let mut card_counts: Vec<u32> = vec![1; line_count];

    for (i, mut row) in input.lines().enumerate() {
        // Get rid of "Card xx: " prefix
        row = row.split(":").nth(1).unwrap();
        let mut row_sections = row.split("|");

        // extract and sort winning numbers
        let mut winning_numbers = extract_numbers(row_sections.next().unwrap());
        winning_numbers.sort();

        // extract and sort actual numbers
        let mut actual_numbers = extract_numbers(row_sections.next().unwrap());
        actual_numbers.sort();

        let mut match_count: usize = 0;

        for num in actual_numbers {
            if let Ok(_) = winning_numbers.binary_search(&num) {
                match_count += 1;
            }
        }

        let multiplier = card_counts[i];
        let next_card = i + 1;
        for card_index in next_card..next_card + match_count {
            card_counts[card_index] += multiplier;
        }
    }

    for ct in card_counts {
        result += ct;
    }

    Ok(result.to_string())
}

fn extract_numbers(input: &str) -> Vec<u8> {
    let mut result = vec![];

    let nums = input.split_whitespace();
    for num in nums {
        let this: u8 = num.parse().unwrap();
        result.push(this);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extract_numbers() {
        {
            let input = "41 48 83 86 17";
            let result = extract_numbers(input);
            let expected = vec![41, 48, 83, 86, 17];
            assert_eq!(result, expected);
        }
    }
    #[test]
    fn integration_test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part1(input);
        assert_eq!(result.unwrap(), "13");
    }
    #[test]
    fn integration_test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part2(input);
        assert_eq!(result.unwrap(), "30");
    }
}
