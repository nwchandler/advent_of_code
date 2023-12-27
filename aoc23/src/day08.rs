use std::collections::HashMap;

pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let mut result: u32 = 0;

    let mut node_map: HashMap<String, (String, String)> = HashMap::new();

    let (directions, nodes) = input.split_once("\n\n").unwrap();

    for n in nodes.split('\n') {
        if n.is_empty() {
            break;
        }
        let mut n_split = n.split(" = ");
        let this_node = n_split.next().unwrap().to_string();
        let lr = n_split.next().unwrap().replace("(", "").replace(")", "");

        let (left, right) = lr.split_once(",").unwrap();

        node_map.insert(
            this_node,
            (left.trim().to_string(), right.trim().to_string()),
        );
    }

    let mut next = String::from("AAA");

    for d in directions.chars().cycle() {
        if next == String::from("ZZZ") {
            break;
        }
        match d {
            'L' => {
                next = node_map.get(&next).unwrap().0.clone();
            }
            'R' => {
                next = node_map.get(&next).unwrap().1.clone();
            }
            _ => panic!("unexpected char: {}", d),
        }
        result += 1;
    }

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
            let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
            let result = part1(input);
            assert_eq!(result.unwrap(), "2")
        }
        {
            let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
            let result = part1(input);
            assert_eq!(result.unwrap(), "6")
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
