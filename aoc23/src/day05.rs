pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

// TODO: Remove allow when implemented
#[allow(unused_variables)]
fn part1(input: &str) -> Result<String, &'static str> {
    let result: u32 = 0;

    Ok(result.to_string())
}

// TODO: Remove allow when implemented
#[allow(unused_variables)]
fn part2(input: &str) -> Result<String, &'static str> {
    let result: u32 = 0;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: don't ignore when implemented
    #[ignore]
    #[test]
    fn integration_test_part1() {
        let input = "";
        let result = part1(input);
        assert_eq!(result.unwrap(), "13");
    }

    // TODO: uncomment when implemented
    // #[ignore]
    // #[test]
    // fn integration_test_part2() {
    //     let input = "";
    //     let result = part2(input);
    //     assert_eq!(result.unwrap(), "??");
    // }
}
