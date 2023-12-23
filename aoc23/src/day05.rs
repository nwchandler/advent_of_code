pub fn run(input: &str) -> Result<crate::Solution, &'static str> {
    Ok(crate::Solution {
        part1: part1(input)?,
        part2: part2(input)?,
    })
}

fn part1(input: &str) -> Result<String, &'static str> {
    let mut result: u64 = std::u64::MAX;
    let mut input_lines = input.split("\n\n");

    // set up a vector of seeds
    let mut seeds: Vec<u64> = vec![];
    let mut seed_input = input_lines.next().unwrap();
    seed_input = seed_input.split(':').nth(1).unwrap();
    seed_input.split_whitespace().for_each(|seed| {
        seeds.push(seed.parse::<u64>().unwrap_or_else(|err| {
            panic!("unable to parse {} from input {}", seed, input);
        }));
    });

    // set up a pipeline of maps
    let mut map_pipeline: Vec<Map> = vec![];
    for map_definition in input_lines.into_iter() {
        let mut map = Map::new();

        for map_entry in map_definition.split('\n').skip(1) {
            map.add_entry(map_entry);
        }
        map_pipeline.push(map);
    }

    for seed in seeds {
        let mut next = seed;
        for map in &map_pipeline {
            next = map.map(next as i64);
        }
        result = if next < result { next } else { result };
    }

    Ok(result.to_string())
}

/// MapEntry corresponds to each map line
#[derive(Debug)]
struct MapEntry {
    destination: i64,
    source: i64,
    range: i64,
}

impl MapEntry {
    fn map_to_dest(&self, source: &i64) -> Option<u64> {
        let range = self.source..self.source + self.range;
        match range.contains(source) {
            true => {
                let res = source - (self.source - self.destination);
                Some(res as u64)
            }
            false => None,
        }
    }
}

/// Map corresponds to an entire map
#[derive(Debug)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn new() -> Map {
        Map { entries: vec![] }
    }

    fn map(&self, input: i64) -> u64 {
        for map_entry in &self.entries {
            if let Some(result) = map_entry.map_to_dest(&input) {
                return result;
            }
        }
        input as u64
    }

    fn add_entry(&mut self, input: &str) {
        let mut numbers: Vec<i64> = vec![];
        input.split_whitespace().for_each(|num| {
            numbers.push(num.parse::<i64>().unwrap_or_else(|err| {
                panic!("unable to parse {} from input {}", num, input);
            }));
        });

        self.entries.push(MapEntry {
            destination: numbers[0],
            source: numbers[1],
            range: numbers[2],
        })
    }
}

fn part2(input: &str) -> Result<String, &'static str> {
    let result: u64 = 0;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_entry() {
        let map_entry = MapEntry {
            destination: 50,
            source: 98,
            range: 2,
        };
        {
            // no match
            let result = map_entry.map_to_dest(&1);
            assert_eq!(result, None);
        }
        {
            // match
            let result = map_entry.map_to_dest(&98);
            assert_eq!(result, Some(50));
        }
    }

    #[test]
    fn test_map() {
        {
            let mut map = Map::new();
            map.add_entry("50 98 2");
            map.add_entry("52 50 48");
            assert_eq!(map.map(98), 50);
            assert_eq!(map.map(50), 52);
            assert_eq!(map.map(10), 10);
        }
        {
            let mut map = Map::new();
            map.add_entry("0 15 37");
            map.add_entry("37 52 2");
            map.add_entry("39 0 15");
            assert_eq!(map.map(14), 53);
        }
    }

    #[test]
    fn integration_test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part1(input);
        assert_eq!(result.unwrap(), "35");
    }

    // TODO: uncomment when implemented
    // #[ignore]
    // #[test]
    // fn integration_test_part2() {
    //     let input = "";
    //     let result = part2(input);
    //     assert_eq!(result.unwrap(), "??");
    // }
}
