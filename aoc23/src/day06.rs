pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let mut result: u32 = 0;

    // create vector of times
    let mut race_times: Vec<u32> = vec![];
    input
        .lines()
        .nth(0)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .for_each(|time| {
            race_times.push(time.parse::<u32>().unwrap_or_else(|err| {
                panic!("unable to parse number from {}, error {}", time, err);
            }))
        });

    // create vector of distances
    let mut race_distances: Vec<u32> = vec![];
    input
        .lines()
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .for_each(|time| {
            race_distances.push(time.parse::<u32>().unwrap_or_else(|err| {
                panic!("unable to parse number from {}, error {}", time, err);
            }))
        });

    for i in 0..race_times.len() {
        let mut hold_time = race_times[i] / 2;

        let mut race_time = if race_times[i] % 2 == 0 {
            race_times[i] / 2
        } else {
            (race_times[i] / 2) + 1
        };

        let mut winning_count = 0;

        while hold_time * race_time > race_distances[i] {
            // When hold time is equal to the remaining race time, there is only one combination
            // that will work; however, when they are different, you could consider the two
            // interchangeable (hold time = 1, race time = 2 is the same as hold time = 2, race
            // time = 1), so we can add 2.
            winning_count += if hold_time != race_time { 2 } else { 1 };
            hold_time -= 1;
            race_time += 1;
        }

        result = if result != 0 {
            result * winning_count
        } else {
            winning_count
        };
    }

    Ok(result.to_string())
}

fn part2(input: &str) -> Result<String, &'static str> {
    let result: u32 = 0;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn integration_test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part1(input);
        assert_eq!(result.unwrap(), "288");
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
