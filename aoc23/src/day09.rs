pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let mut result: i32 = 0;

    for sequence in input.lines() {
        let mut this_seq: Vec<i32> = vec![];
        for number in sequence.split_whitespace() {
            this_seq.push(number.to_string().parse::<i32>().unwrap());
        }
        result += derive_next_from_seq(&this_seq);
    }

    Ok(result.to_string())
}

fn part2(input: &str) -> Result<String, &'static str> {
    let mut result: i32 = 0;

    for sequence in input.lines() {
        let mut this_seq: Vec<i32> = vec![];
        for number in sequence.split_whitespace() {
            this_seq.push(number.to_string().parse::<i32>().unwrap());
        }
        this_seq.reverse();
        result += derive_next_from_seq(&this_seq);
    }

    Ok(result.to_string())
}

fn derive_next_from_seq(input: &Vec<i32>) -> i32 {
    let mut zeroes = 0;
    // check if there are any non-zero inputs
    for i in input {
        zeroes |= i;
    }
    // if zeroes does not still equal 0, then at least one of the inputs was nonzero
    if zeroes != 0 {
        let mut next_seq: Vec<i32> = vec![];
        let mut rev = input.clone();
        rev.reverse();
        let last_number = rev[0];
        for i in 0..rev.len() - 1 {
            let highest = rev[i];
            let next_highest = rev[i + 1];
            next_seq.push(highest - next_highest);
        }
        next_seq.reverse();
        let next = derive_next_from_seq(&next_seq);
        return last_number + next;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_next_from_seq() {
        {
            let input = vec![0, 3, 6, 9, 12, 15];
            let result = derive_next_from_seq(&input);
            assert_eq!(result, 18);
        }
        {
            let input = vec![1, 3, 6, 10, 15, 21];
            let result = derive_next_from_seq(&input);
            assert_eq!(result, 28);
        }
        {
            let input = vec![10, 13, 16, 21, 30, 45];
            let result = derive_next_from_seq(&input);
            assert_eq!(result, 68);
        }
    }

    #[test]
    fn integration_test_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = part1(input);
        assert_eq!(result.unwrap(), "114");
    }

    #[test]
    fn integration_test_part2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = part2(input);
        assert_eq!(result.unwrap(), "2");
    }
}
