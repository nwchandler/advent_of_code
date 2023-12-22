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

        // extract and sort winning numbers
        let mut winning_numbers = extract_numbers(row_sections.next().unwrap());
        winning_numbers.sort();

        // extract and sort actual numbers
        let mut actual_numbers = extract_numbers(row_sections.next().unwrap());
        actual_numbers.sort();

        let mut match_count: u32 = 0;

        for num in actual_numbers {
            if let Ok(_) = winning_numbers.binary_search(&num) {
                match_count += 1;
            }
        }

        // If there are no matches, (1 << 0) is 1, and we do an integer division by 2, which will
        // yield 0 to add to the total result.
        result += (1 << match_count) / 2;
    }

    Ok(result.to_string())
}

fn part2(_input: &str) -> Result<String, &'static str> {
    let result: u32 = 0;

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
}
