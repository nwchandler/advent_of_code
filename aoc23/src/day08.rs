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

fn part2(input: &str) -> Result<String, &'static str> {
    let mut result: usize = 1;

    let mut node_map: HashMap<String, (String, String)> = HashMap::new();

    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let mut next_vec: Vec<String> = vec![];
    let mut path_counts: Vec<usize> = vec![];

    for n in nodes.split('\n') {
        if n.is_empty() {
            break;
        }
        let mut n_split = n.split(" = ");
        let this_node = n_split.next().unwrap().to_string();
        let lr = n_split.next().unwrap().replace("(", "").replace(")", "");

        let (left, right) = lr.split_once(",").unwrap();

        node_map.insert(
            this_node.clone(),
            (left.trim().to_string(), right.trim().to_string()),
        );

        if this_node.ends_with('A') {
            next_vec.push(this_node);
            path_counts.push(0);
        }
    }

    for i in 0..next_vec.len() {
        let mut counter: usize = 0;
        for d in directions.chars().cycle() {
            counter += 1;
            match d {
                'L' => {
                    next_vec[i] = node_map.get(&next_vec[i]).unwrap().0.clone();
                }
                'R' => {
                    next_vec[i] = node_map.get(&next_vec[i]).unwrap().1.clone();
                }
                _ => panic!("unexpected char: {}", d),
            }
            if next_vec[i].ends_with('Z') {
                path_counts[i] = counter;
                break;
            }
        }
    }

    for i in 0..path_counts.len() {
        result = lcm(result, path_counts[i]);
    }

    Ok(result.to_string())
}

// "borrowed" implementation of least common multiple from https://rustp.org/number-theory/lcm/
fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    // LCM = a*b / gcd
    return a * (b / gcd(a, b));
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

    #[test]
    fn integration_test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part2(input);
        assert_eq!(result.unwrap(), "6");
    }
}
