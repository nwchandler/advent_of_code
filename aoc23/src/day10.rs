use std::collections::VecDeque;

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

fn part2(input: &str) -> Result<String, &'static str> {
    let mut result = 0;

    let lines = input.lines();
    let row_count: usize = lines.clone().count();
    let row_length: usize = lines.clone().next().unwrap().chars().count();
    let mut source_map: Vec<Vec<u8>> = vec![vec![0; row_length * 3]; row_count * 3];

    let mut s: (usize, usize) = (0, 0);
    for (i, line) in lines.enumerate() {
        let this_row = i * 3 + 1;
        for (j, c) in line.chars().enumerate() {
            let this_col = j * 3 + 1;
            source_map[this_row][this_col] = 1;
            match c {
                '|' => {
                    source_map[this_row - 1][this_col] = 1;
                    source_map[this_row + 1][this_col] = 1;
                }
                '-' => {
                    source_map[this_row][this_col - 1] = 1;
                    source_map[this_row][this_col + 1] = 1;
                }
                'L' => {
                    source_map[this_row - 1][this_col] = 1;
                    source_map[this_row][this_col + 1] = 1;
                }
                'J' => {
                    source_map[this_row - 1][this_col] = 1;
                    source_map[this_row][this_col - 1] = 1;
                }
                '7' => {
                    source_map[this_row][this_col - 1] = 1;
                    source_map[this_row + 1][this_col] = 1;
                }
                'F' => {
                    source_map[this_row][this_col + 1] = 1;
                    source_map[this_row + 1][this_col] = 1;
                }
                '.' => {
                    source_map[this_row][this_col] = 0;
                }
                'S' => {
                    s = (this_row, this_col);
                }
                _ => panic!("unexpected character: {c}"),
            }
        }
    }

    // fill in for s
    if s.1 > 2 && source_map[s.0][(s.1) - 2] == 1 {
        source_map[s.0][(s.1) - 1] = 1;
    }
    if source_map[s.0][(s.1) + 2] == 1 {
        source_map[s.0][(s.1) + 1] = 1;
    }
    if s.0 > 2 && source_map[(s.0) - 2][s.1] == 1 {
        source_map[(s.0) - 1][s.1] = 1;
    }
    if source_map[(s.0) + 2][s.1] == 1 {
        source_map[(s.0) + 1][s.1] = 1;
    }

    let mut calculation_matrix: Vec<Vec<u8>> = vec![vec![0; row_length * 3]; row_count * 3];

    let mut last_visited = s;
    let mut next = s;
    loop {
        let this_row = next.0;
        let this_col = next.1;
        calculation_matrix[this_row][this_col] = 1;

        let neighbor_left = (this_row, this_col - 1);
        let neighbor_right = (this_row, this_col + 1);
        let neighbor_up = (this_row - 1, this_col);
        let neighbor_down = (this_row + 1, this_col);

        if last_visited != neighbor_left && source_map[neighbor_left.0][neighbor_left.1] == 1 {
            next = neighbor_left;
        }
        if last_visited != neighbor_right && source_map[neighbor_right.0][neighbor_right.1] == 1 {
            next = neighbor_right;
        }
        if last_visited != neighbor_up && source_map[neighbor_up.0][neighbor_up.1] == 1 {
            next = neighbor_up;
        }
        if last_visited != neighbor_down && source_map[neighbor_down.0][neighbor_down.1] == 1 {
            next = neighbor_down;
        }

        last_visited = (this_row, this_col);
        // once we get back to s, we have completed the loop
        if next == s {
            break;
        }
    }

    let mut flood_queue: VecDeque<(usize, usize)> = VecDeque::from([(0, 0)]);
    while flood_queue.len() != 0 {
        let next = flood_queue.pop_back().unwrap();
        let this_row = next.0;
        let this_col = next.1;
        calculation_matrix[this_row][this_col] = 2;

        let neighbor_left = if this_col > 0 {
            (this_row, this_col - 1)
        } else {
            (this_row, this_col)
        };
        if calculation_matrix[neighbor_left.0][neighbor_left.1] == 0 {
            flood_queue.push_back(neighbor_left)
        }

        let neighbor_right = if this_col < calculation_matrix[0].len() - 1 {
            (this_row, this_col + 1)
        } else {
            (this_row, this_col)
        };
        if calculation_matrix[neighbor_right.0][neighbor_right.1] == 0 {
            flood_queue.push_back(neighbor_right)
        }

        let neighbor_up = if this_row > 0 {
            (this_row - 1, this_col)
        } else {
            (this_row, this_col)
        };
        if calculation_matrix[neighbor_up.0][neighbor_up.1] == 0 {
            flood_queue.push_back(neighbor_up)
        }

        let neighbor_down = if this_row < calculation_matrix.len() - 1 {
            (this_row + 1, this_col)
        } else {
            (this_row, this_col)
        };
        if calculation_matrix[neighbor_down.0][neighbor_down.1] == 0 {
            flood_queue.push_back(neighbor_down)
        }
    }

    // NOTE: I took the calculation approach because I like to see the graph. :)
    for (i, row) in calculation_matrix.into_iter().enumerate() {
        for (j, col) in row.into_iter().enumerate() {
            if col == 0 {
                // Each "real" node is centered in a 3x3 grid, so to calculate the interior nodes,
                // we need to only look at these center nodes. There will still be individual cells
                // in the calculation matrix whose values are 0, but if they aren't centered, we
                // should not count them.
                if j % 3 == 1 && i % 3 == 1 {
                    result += 1;
                    print!("\x1b[47m \x1b[0m");
                } else {
                    print!(" ");
                }
            } else if col == 2 {
                print!("\x1b[38;5;159m·\x1b[0m");
            } else {
                print!("\x1b[48;5;240m \x1b[0m");
            }
        }
        println!();
    }

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

    #[test]
    fn integration_test_part2() {
        {
            let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
            let result = part2(input);
            assert_eq!(result.unwrap(), "4");
        }
        {
            let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
            let result = part2(input);
            assert_eq!(result.unwrap(), "8");
        }
        {
            let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
            let result = part2(input);
            assert_eq!(result.unwrap(), "10");
        }
    }
}
