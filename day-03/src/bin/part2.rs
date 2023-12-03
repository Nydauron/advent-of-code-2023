use std::collections::{BTreeMap, HashMap};

use nom::{character, IResult};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("input.txt");
    println!("{}", part2(input))
}

fn get_number(input: &str) -> IResult<&str, u32> {
    let (input, number) = character::complete::u32(input)?;
    Ok((input, number))
}

fn is_symbol(c: char) -> bool {
    c == '*'
}

fn get_gear_ratio(
    schematic: &BTreeMap<(usize, usize), PointType>,
    symbol_location: (usize, usize),
) -> Option<u64> {
    let gear_ratio_req_count = 2;
    let positions = [
        (symbol_location.0 - 1, symbol_location.1 - 1),
        (symbol_location.0 - 1, symbol_location.1),
        (symbol_location.0 - 1, symbol_location.1 + 1),
        (symbol_location.0, symbol_location.1 - 1),
        (symbol_location.0, symbol_location.1 + 1),
        (symbol_location.0 + 1, symbol_location.1 - 1),
        (symbol_location.0 + 1, symbol_location.1),
        (symbol_location.0 + 1, symbol_location.1 + 1),
    ];
    let seen_numbers = positions
        .iter()
        .filter_map(|pos| {
            let cell = schematic.get(pos);
            let cell = cell?;
            if let PointType::Number(pos, value) = cell {
                Some((*pos, *value))
            } else {
                None
            }
        })
        .fold(HashMap::new(), |mut acc, (pos, val)| {
            acc.entry(pos).or_insert(val);
            acc
        });

    if seen_numbers.len() == gear_ratio_req_count {
        Some(seen_numbers.values().map(|val| *val as u64).product())
    } else {
        None
    }
}

enum PointType {
    Symbol,
    Number((usize, usize), u32),
}

fn part2(input: &str) -> u64 {
    let lines = input.lines();
    let schematic: BTreeMap<(usize, usize), PointType> = lines
        .enumerate()
        .flat_map(|(row_idx, line)| {
            let mut digit_location: Option<((usize, usize), u32)> = None;
            line.chars()
                .enumerate()
                .filter_map(move |(col_idx, c)| {
                    if c.is_ascii_digit() {
                        if let Some((location, value)) = digit_location {
                            Some(((row_idx, col_idx), PointType::Number(location, value)))
                        } else {
                            let (_, num) = get_number(&line[col_idx..]).expect("no number");
                            digit_location = Some(((row_idx, col_idx), num));
                            Some((
                                (row_idx, col_idx),
                                PointType::Number((row_idx, col_idx), num),
                            ))
                        }
                    } else if is_symbol(c) {
                        digit_location = None;
                        Some(((row_idx, col_idx), PointType::Symbol))
                    } else {
                        digit_location = None;
                        None
                    }
                })
                // 1-index the positions to prevent overflow errors when looking around symbols
                .map(|(pos, mut point)| {
                    if let PointType::Number(pos, val) = point {
                        point = PointType::Number((pos.0 + 1, pos.1 + 1), val);
                    }
                    ((pos.0 + 1, pos.1 + 1), point)
                })
        })
        .collect::<BTreeMap<_, _>>();

    schematic
        .iter()
        .filter_map(|(pos, cell)| match cell {
            PointType::Symbol => get_gear_ratio(&schematic, *pos),
            _ => None,
        })
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
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
        assert_eq!(part2(input), 467835);
    }
}
