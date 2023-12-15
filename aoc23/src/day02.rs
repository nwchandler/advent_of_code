use std::collections::HashMap;

pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let mut result: usize = 0;
    let max = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    for (game_idx, input) in input.lines().enumerate() {
        let mut line_sections = input.split(':');
        if part1_check_game(line_sections.nth(1).unwrap(), &max) {
            result = result + game_idx + 1;
        }
    }

    Ok(result.to_string())
}

fn part2(input: &str) -> Result<String, &'static str> {
    let mut result: u32 = 0;
    for line in input.lines() {
        let mut line_sections = line.split(':');
        result += part2_check_game(line_sections.nth(1).unwrap());
    }

    Ok(result.to_string())
}

fn part1_check_game(input: &str, max: &HashMap<&str, u32>) -> bool {
    let pairs = input.split(|c| matches!(c, ';' | ','));
    for mut pair in pairs {
        pair = pair.trim();

        let mut pair_split = pair.split(' ');
        // at this point, we should have two elements, a count and a color
        let count: u32;
        let color: &str;
        if let Some(i) = pair_split.next() {
            count = i.parse().unwrap();
        } else {
            return false;
        }
        if let Some(i) = pair_split.next() {
            color = i;
        } else {
            return false;
        }

        if let Some(c) = max.get(&color) {
            if &count > c {
                // if the count pulled is greater than max, it is not possible
                return false;
            }
        } else {
            // if the color is not in the max at all, it is not possible
            return false;
        }
    }

    // if we've made it through all the checks, then this game is possible
    true
}

// TODO: disable dead code check
#[allow(dead_code)]
fn part2_check_game(input: &str) -> u32 {
    let mut min = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    let pairs = input.split(|c| matches!(c, ';' | ','));
    for mut pair in pairs {
        pair = pair.trim();

        let mut pair_split = pair.split(' ');
        let count: u32 = pair_split.next().unwrap().parse().unwrap();
        let color: &str = pair_split.next().unwrap();

        let current_min = min.get(&color).unwrap();
        if &count > current_min {
            min.insert(color, count);
        }
    }

    let min_red = min.get("red").unwrap();
    let min_green = min.get("green").unwrap();
    let min_blue = min.get("blue").unwrap();

    min_red * min_green * min_blue
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_check_game() {
        {
            // Game 1
            let max = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
            let result = part1_check_game("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", &max);
            assert!(result);
        }
        {
            // Game 2
            let max = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
            let result = part1_check_game(
                "1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                &max,
            );
            assert!(result);
        }
        {
            // Game 3
            let max = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
            let result = part1_check_game(
                "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                &max,
            );
            assert!(!result);
        }
        {
            // Game 4
            let max = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
            let result = part1_check_game(
                "1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                &max,
            );
            assert!(!result);
        }
        {
            // Game 5
            let max = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
            let result = part1_check_game("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", &max);
            assert!(result);
        }
    }

    #[test]
    fn test_part2_check_game() {
        {
            // Game 1
            let result = part2_check_game("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
            assert_eq!(result, 48);
        }
        {
            // Game 2
            let result =
                part2_check_game("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
            assert_eq!(result, 12);
        }
        {
            // Game 3
            let result = part2_check_game(
                "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            );
            assert_eq!(result, 1560);
        }
        {
            // Game 4
            let result = part2_check_game(
                "1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            );
            assert_eq!(result, 630);
        }
        {
            // Game 5
            let result = part2_check_game("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
            assert_eq!(result, 36);
        }
    }
}
