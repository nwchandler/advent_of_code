pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let result;

    let mut map: Vec<(usize, usize)> = vec![];
    let mut row_length: usize = 0;

    let mut s: usize = 0;
    for (i, line) in input.lines().enumerate() {
        // we'll use the row length for offsets later
        if row_length == 0 {
            row_length = line.chars().count();
        }
        for (j, c) in line.chars().enumerate() {
            // this is the position of the character in the array
            let this = j + (i * row_length);

            // since we're using unsigned numbers here, we need to avoid underruns
            let pipe_north = if this > row_length {
                this - row_length
            } else {
                this
            };
            let pipe_west = if this % row_length > 0 {
                this - 1
            } else {
                this
            };

            // we're taking a short cut and ignoring going past the right or bottom of the map
            let pipe_south = this + row_length;
            let pipe_east = this + 1;

            map.push(match c {
                '|' => (pipe_north, pipe_south),
                '-' => (pipe_west, pipe_east),
                'L' => (pipe_north, pipe_east),
                'J' => (pipe_north, pipe_west),
                '7' => (pipe_south, pipe_west),
                'F' => (pipe_south, pipe_east),
                // . doesn't go anyplace...
                '.' => (this, this),
                // this is going to get changed later on
                'S' => {
                    s = this;
                    (this, this)
                }
                _ => panic!("unexpected character: {c}"),
            });
        }
    }

    {
        let elem_above_s = s - row_length;
        let elem_below_s = s + row_length;
        let elem_left_s = s - 1;
        let elem_right_s = s + 1;

        let mut first: usize = 0;
        let mut second: usize = 0;
        if s == map[elem_above_s].0 || s == map[elem_above_s].1 {
            match first {
                0 => first = elem_above_s,
                _ => second = elem_above_s,
            }
        }
        if s == map[elem_below_s].0 || s == map[elem_below_s].1 {
            match first {
                0 => first = elem_below_s,
                _ => second = elem_below_s,
            }
        }
        if s == map[elem_left_s].0 || s == map[elem_left_s].1 {
            match first {
                0 => first = elem_left_s,
                _ => second = elem_left_s,
            }
        }
        if s == map[elem_right_s].0 || s == map[elem_right_s].1 {
            match first {
                0 => first = elem_right_s,
                _ => second = elem_right_s,
            }
        }
        map[s].0 = first;
        map[s].1 = second;
    }

    let mut pipe_length = 1;
    let mut last_visited = s;
    let mut next = map[s].0;
    loop {
        let this = next;
        if last_visited == map[this].0 {
            next = map[this].1;
        } else {
            next = map[this].0;
        };
        last_visited = this;
        pipe_length += 1;
        // once we get back to s, we have completed the loop
        if next == s {
            break;
        }
    }

    // the farthest point of the loop will be 1/2 the total distance away
    result = pipe_length / 2;

    Ok(result.to_string())
}

fn part2(_input: &str) -> Result<String, &'static str> {
    let result: u32 = 0;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_test_part1() {
        {
            let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
            let result = part1(input);
            assert_eq!(result.unwrap(), "4");
        }
        {
            let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
            let result = part1(input);
            assert_eq!(result.unwrap(), "8");
        }
    }

    // TODO: Don't ignore once implemented
    #[ignore]
    #[test]
    fn integration_test_part2() {
        let input = "";
        let result = part2(input);
        assert_eq!(result.unwrap(), "??");
    }
}
