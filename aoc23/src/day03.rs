use std::fmt;

pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let mut result: u32 = 0;
    let schematic = build_schematic_from_input(input);

    for number in schematic.numbers {
        let row_above = if number.row > 0 { number.row - 1 } else { 0 };
        let row_below = number.row + 1;

        let col_before = if number.col > 0 { number.col - 1 } else { 0 };
        let col_after = number.col + number.number.len() + 1;

        for row in row_above..row_below + 1 {
            if let Some(r) = schematic.symbol_locations.get(row) {
                for col in col_before..col_after {
                    if let Some(c) = r.get(col) {
                        if c == &1 {
                            let this: u32 = number.number.parse().unwrap();
                            result += this;
                        }
                    }
                }
            }
        }
    }

    Ok(result.to_string())
}

fn part2(_input: &str) -> Result<String, &'static str> {
    let result: usize = 0;
    Ok(result.to_string())
}

struct Schematic {
    symbol_locations: Vec<Vec<u8>>,
    numbers: Vec<Number>,
}

#[derive(PartialEq, Debug, Clone)]
struct Number {
    number: String,
    row: usize,
    col: usize,
}

impl fmt::Display for Number {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}

fn build_schematic_from_input(input: &str) -> Schematic {
    // The fact that there IS an input implies the existence of a row; however,
    // in the event there are no characters, there may not be columns. We initialize
    // the vector for row 0 here, but we'll fill it out further as we parse the input.
    let mut symbol_locations = vec![];
    let mut numbers = vec![];

    for (row, mut line) in input.lines().enumerate() {
        // get rid of incidental extra spaces
        line = line.trim();
        let mut column = vec![];

        let mut current_number: Option<Number> = None;

        for (col, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if let Some(ref mut n) = current_number {
                    n.number.push(c);
                } else {
                    current_number = Some(Number {
                        number: String::from(c),
                        row,
                        col,
                    });
                }
                column.push(0);
                continue;
            }

            if c == '.' {
                column.push(0);
            } else {
                column.push(1);
            }
            if let Some(ref n) = &current_number {
                numbers.push(n.clone());
            }
            current_number = None;
        }

        if let Some(ref n) = &current_number {
            numbers.push(n.clone());
        }
        symbol_locations.push(column);
    }

    Schematic {
        symbol_locations,
        numbers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        {
            // empty input
            let input = "";

            let expected_matrix: Vec<Vec<u8>> = vec![];
            let expected_numbers = vec![];
            let result = build_schematic_from_input(input);
            assert_eq!(result.numbers, expected_numbers);
            assert_eq!(result.symbol_locations, expected_matrix);
        }
        {
            // only numbers
            let input = "1..
                        ...
                        .23";

            let expected_matrix: Vec<Vec<u8>> = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
            let expected_numbers = vec![
                Number {
                    number: String::from("1"),
                    row: 0,
                    col: 0,
                },
                Number {
                    number: String::from("23"),
                    row: 2,
                    col: 1,
                },
            ];
            let result = build_schematic_from_input(input);
            assert_eq!(result.numbers, expected_numbers);
            assert_eq!(result.symbol_locations, expected_matrix);
        }
        {
            // only symbols
            let input = "..*
                        @.*
                        ...";

            let expected_matrix = vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]];
            let expected_numbers = vec![];
            let result = build_schematic_from_input(input);
            assert_eq!(result.numbers, expected_numbers);
            assert_eq!(result.symbol_locations, expected_matrix);
        }
        {
            // symbols and numbers
            let input = "..*1.
                        @.*.2
                        12..9";

            let expected_matrix = vec![
                vec![0, 0, 1, 0, 0],
                vec![1, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0],
            ];
            let expected_numbers = vec![
                Number {
                    number: String::from("1"),
                    row: 0,
                    col: 3,
                },
                Number {
                    number: String::from("2"),
                    row: 1,
                    col: 4,
                },
                Number {
                    number: String::from("12"),
                    row: 2,
                    col: 0,
                },
                Number {
                    number: String::from("9"),
                    row: 2,
                    col: 4,
                },
            ];
            let result = build_schematic_from_input(input);
            assert_eq!(result.numbers, expected_numbers);
            assert_eq!(result.symbol_locations, expected_matrix);
        }
    }

    #[test]
    fn integration_test_part1() {
        let input = "467..114..
                    ...*......
                    ..35..633.
                    ......#...
                    617*......
                    .....+.58.
                    ..592.....
                    ......755.
                    ...$.*....
                    .664.598..";
        let result = part1(input);
        assert_eq!(result.unwrap(), "4361");
    }
}
