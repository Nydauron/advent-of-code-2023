use std::ops::RangeInclusive;

use range_set::RangeSet;

pub fn part2(input: &str) -> u64 {
    let almanac = parse_input(input);
    let ranges = almanac
        .initial_seeds
        .iter()
        .map(|seeds| seeds.start_seed..=(seeds.start_seed + seeds.range - 1))
        .collect::<Vec<_>>();

    let mut val = RangeSet::<[RangeInclusive<u64>; 4096]>::new();
    for r in ranges {
        val.insert_range(r);
    }

    for mapping_layer in almanac.mappings {
        let mut new_ranges = RangeSet::<[RangeInclusive<u64>; 4096]>::new();
        for map in &mapping_layer {
            let map_range = map.start_source..=(map.start_source + map.range - 1);

            let removed_range_set = val.remove_range(map_range);

            if let Some(removed_range_set) = removed_range_set {
                for range in removed_range_set.as_ref().iter().map(|range| {
                    let start = range.start();
                    let end = range.end();
                    (start - map.start_source + map.start_dest)
                        ..=(end - map.start_source + map.start_dest)
                }) {
                    new_ranges.insert_range(range);
                }
            }
        }

        for range in val.as_ref().iter() {
            new_ranges.insert_range(range.clone());
        }
        val = new_ranges;
    }
    val.min().unwrap()
}

#[derive(Debug, Default, Copy, Clone)]
struct Mapping {
    start_source: u64,
    start_dest: u64,
    range: u64,
}

#[derive(Debug, Default, Copy, Clone)]
struct SeedRange {
    start_seed: u64,
    range: u64,
}

#[derive(Debug, Default, Clone)]
struct Almanac {
    initial_seeds: Vec<SeedRange>,
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
                let parsed_numbers = numbers
                    .trim()
                    .split_whitespace()
                    .map(|n| {
                        n.parse::<u64>()
                            .expect("value in numbers array was not a number")
                    })
                    .collect::<Vec<_>>();
                let ranges = parsed_numbers.iter().enumerate().filter_map(|(idx, n)| {
                    if idx % 2 == 0 {
                        None
                    } else {
                        Some(n)
                    }
                });

                let start_seed = parsed_numbers.iter().enumerate().filter_map(|(idx, n)| {
                    if idx % 2 == 0 {
                        Some(n)
                    } else {
                        None
                    }
                });

                almanac.initial_seeds = start_seed
                    .zip(ranges)
                    .map(|(start, range)| SeedRange {
                        start_seed: *start,
                        range: *range,
                    })
                    .collect::<Vec<_>>()
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
    fn test_part2() {
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

        assert_eq!(part2(input), 46);
    }
}
