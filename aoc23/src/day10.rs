pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let result: u32 = 0;

    let mut map: Vec<Vec<u8>> = vec![];

    let mut s: (usize, usize) = (0, 0);
    for (i, line) in input.lines().enumerate() {
        let mut this_row: Vec<u8> = vec![];
        for (j, c) in line.chars().enumerate() {
            this_row.push(match c {
                '|' => Direction::NORTH | Direction::SOUTH,
                '-' => Direction::EAST | Direction::WEST,
                'L' => Direction::NORTH | Direction::EAST,
                'J' => Direction::NORTH | Direction::WEST,
                '7' => Direction::SOUTH | Direction::WEST,
                'F' => Direction::SOUTH | Direction::EAST,
                '.' => 0,
                'S' => {
                    s = (i, j);
                    0
                }
                _ => panic!("unexpected character: {c}"),
            });
        }
        map.push(this_row);
    }

    {
        // taking a shortcut here and assuming S is not on an edge row or column
        let elem_above_s = map[s.0 - 1][s.1];
        let elem_below_s = map[s.0 + 1][s.1];
        let elem_left_of_s = map[s.0][s.1 - 1];
        let elem_right_of_s = map[s.0][s.1 + 1];

        if (elem_above_s & Direction::SOUTH) != 0 {
            map[s.0][s.1] |= Direction::NORTH;
        }
        if (elem_below_s & Direction::NORTH) != 0 {
            map[s.0][s.1] |= Direction::SOUTH;
        }
        if (elem_left_of_s & Direction::EAST) != 0 {
            map[s.0][s.1] |= Direction::WEST;
        }
        if (elem_right_of_s & Direction::WEST) != 0 {
            map[s.0][s.1] |= Direction::EAST;
        }
    }

    let mut pipe_length = 0;

    // initialize pipe length to 0
    // set last visited
    // while the next node is not S {
    //      add 1 to pipe length
    //      set next node to
    // }

    Ok(result.to_string())
}

fn part2(_input: &str) -> Result<String, &'static str> {
    let result: u32 = 0;

    Ok(result.to_string())
}

enum Direction {}

impl Direction {
    const NORTH: u8 = 1;
    const SOUTH: u8 = 2;
    const EAST: u8 = 4;
    const WEST: u8 = 8;
}

// #[derive(Copy, Clone, Debug)]
// #[repr(u8)]
// enum Pipe {
//     // |
//     Vertical = Direction::NORTH | Direction::SOUTH,
//     // -
//     Horizontal = Direction::WEST | Direction::EAST,
//     // L
//     L = Direction::NORTH | Direction::EAST,
//     // J
//     J = Direction::NORTH | Direction::WEST,
//     // 7
//     Seven = Direction::SOUTH | Direction::WEST,
//     // F
//     F = Direction::SOUTH | Direction::EAST,
// }
//
// impl Pipe {
//     fn match_direction(&self, direction: u8) -> bool {
//         (*self as u8 & direction) != 0
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Don't ignore once implemented
    #[ignore]
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
