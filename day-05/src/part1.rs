pub fn part1(input: &str) -> u64 {
    let almanac = parse_input(input);

    let mut val = almanac.initial_seeds;
    for mapping_layer in almanac.mappings {
        val = val
            .iter()
            .map(|v| {
                for map in &mapping_layer {
                    if (map.start_source..(map.start_source + map.range)).contains(v) {
                        return *v - map.start_source + map.start_dest;
                    }
                }
                return *v;
            })
            .collect::<Vec<_>>()
    }
    *val.iter().min().unwrap()
}

#[derive(Debug, Default, Copy, Clone)]
struct Mapping {
    start_source: u64,
    start_dest: u64,
    range: u64,
}

#[derive(Debug, Default, Clone)]
struct Almanac {
    initial_seeds: Vec<u64>,
    mappings: Vec<Vec<Mapping>>,
}

fn parse_mappings(lines: &str) -> Vec<Mapping> {
    let line_mappings = lines.trim().lines();
    let mappings = line_mappings
        .map(|line| {
            let mut numbers_for_mapping = line.split_whitespace().map(|n| {
                n.parse::<u64>()
                    .expect("value in numbers array was not a number")
            });
            Mapping {
                start_dest: numbers_for_mapping
                    .next()
                    .expect("Number not found for mapping"),
                start_source: numbers_for_mapping
                    .next()
                    .expect("Number not found for mapping"),
                range: numbers_for_mapping
                    .next()
                    .expect("Number not found for mapping"),
            }
        })
        .collect::<Vec<_>>();
    mappings
}
fn parse_input(input: &str) -> Almanac {
    let groupings = input.split("\n\n").collect::<Vec<_>>();

    let mut almanac = Almanac::default();

    for group in groupings {
        let (group_name, numbers) = group
            .split_once(":")
            .expect("grouping does not have a : delimiter");
        match group_name {
            "seeds" => {
                almanac.initial_seeds = numbers
                    .trim()
                    .split_whitespace()
                    .map(|n| {
                        n.parse::<u64>()
                            .expect("value in numbers array was not a number")
                    })
                    .collect::<Vec<_>>();
            }
            "seed-to-soil map"
            | "soil-to-fertilizer map"
            | "fertilizer-to-water map"
            | "water-to-light map"
            | "light-to-temperature map"
            | "temperature-to-humidity map"
            | "humidity-to-location map" => {
                almanac.mappings.push(parse_mappings(numbers));
            }
            _ => panic!("unknown grouping found"),
        }
    }

    almanac
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
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

        assert_eq!(part1(input), 35);
    }
}
