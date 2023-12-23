pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let result: u32 = 0;

    Ok(result.to_string())
}

fn part2(input: &str) -> Result<String, &'static str> {
    let result: u32 = 0;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Don't ignore after implementing
    #[ignore]
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
