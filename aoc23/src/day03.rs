use std::fmt;

pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let mut result: u32 = 0;
    let filter_part = None;
    let schematic = build_schematic_from_input(input, filter_part);

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

#[derive(Debug)]
struct Gear {
    part_numbers: Vec<Number>,
}

impl Gear {
    fn new() -> Gear {
        Gear {
            part_numbers: vec![],
        }
    }

    fn add_adjacent_number(&mut self, number: &Number) {
        self.part_numbers.push(number.clone());
    }

    fn ratio(&self) -> u32 {
        match self.part_numbers.len() {
            2 => {
                let first: u32 = self.part_numbers.get(0).unwrap().number.parse().unwrap();
                let second: u32 = self.part_numbers.get(1).unwrap().number.parse().unwrap();
                first * second
            }
            _ => 0,
        }
    }
}

fn part2(input: &str) -> Result<String, &'static str> {
    let mut result: u32 = 0;
    let filter_part = Some('*');
    let schematic = build_schematic_from_input(input, filter_part);

    let mut gear_matrix: Vec<Vec<Option<Gear>>> = vec![];

    for row in &schematic.symbol_locations {
        let mut gear_row: Vec<Option<Gear>> = vec![];
        for col in row {
            if col == &1 {
                gear_row.push(Some(Gear::new()));
            } else {
                gear_row.push(None);
            }
        }
        gear_matrix.push(gear_row);
    }

    for number in schematic.numbers {
        let row_above = if number.row > 0 { number.row - 1 } else { 0 };
        let row_below = number.row + 1;

        let col_before = if number.col > 0 { number.col - 1 } else { 0 };
        let col_after = number.col + number.number.len() + 1;

        for row in row_above..row_below + 1 {
            if let Some(r) = gear_matrix.get_mut(row) {
                for col in col_before..col_after {
                    if let Some(c) = r.get_mut(col) {
                        if let Some(gear) = c {
                            gear.add_adjacent_number(&number);
                        }
                    }
                }
            }
        }
    }

    for row in gear_matrix {
        for cell in row {
            if let Some(gear) = cell {
                result += gear.ratio();
            }
        }
    }
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

/// build_schematic_from_input takes an input string representation of the schematic
/// and produces a Schematic struct. If a filter_part is provided, then the Schematic's
/// symbol locations matrix will only have 1s where the filtered part is present.
fn build_schematic_from_input(input: &str, filter_part: Option<char>) -> Schematic {
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
                match filter_part {
                    Some(f) => {
                        if c == f {
                            column.push(1);
                        } else {
                            column.push(0);
                        }
                    }
                    None => column.push(1),
                };
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
    fn test_build_schematic_from_input() {
        {
            // empty input
            let input = "";

            let expected_matrix: Vec<Vec<u8>> = vec![];
            let expected_numbers = vec![];
            let filter_part = None;
            let result = build_schematic_from_input(input, filter_part);
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
            let filter_part = None;
            let result = build_schematic_from_input(input, filter_part);
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
            let filter_part = None;
            let result = build_schematic_from_input(input, filter_part);
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
            let filter_part = None;
            let result = build_schematic_from_input(input, filter_part);
            assert_eq!(result.numbers, expected_numbers);
            assert_eq!(result.symbol_locations, expected_matrix);
        }
        {
            // filtered symbols
            let input = "..*
                        @.*
                        ...";

            let expected_matrix = vec![vec![0, 0, 1], vec![0, 0, 1], vec![0, 0, 0]];
            let expected_numbers = vec![];
            let filter_part = Some('*');
            let result = build_schematic_from_input(input, filter_part);
            assert_eq!(result.numbers, expected_numbers);
            assert_eq!(result.symbol_locations, expected_matrix);
        }
        {
            // filtered symbols and numbers
            let input = "..*1.
                        @.*.2
                        12..9";

            let expected_matrix = vec![
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
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
            let filter_part = Some('*');
            let result = build_schematic_from_input(input, filter_part);
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

    #[test]
    fn integration_test_part2() {
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
        let result = part2(input);
        assert_eq!(result.unwrap(), "467835");
    }
}
