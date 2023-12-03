use std::collections::HashMap;

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

fn get_gear_ratio(schematic: &Vec<Vec<PointType>>, symbol_location: (usize, usize)) -> Option<u64> {
    let mut seen_numbers = HashMap::new();
    let gear_ratio_req_count = 2;
    for row in schematic
        .iter()
        .take((schematic.len() - 1).min(symbol_location.0 + 1))
        .skip(0.max(symbol_location.0 as i64 - 1) as usize)
    {
        for cell in row
            .iter()
            .take((schematic.len() - 1).min(symbol_location.1 + 1))
            .skip(0.max(symbol_location.1 as i64 - 1) as usize)
        {
            if let PointType::Number(pos, value) = cell {
                seen_numbers.entry(pos).or_insert(*value);
            }
        }
    }
    if seen_numbers.len() == gear_ratio_req_count {
        Some(seen_numbers.values().map(|val| *val as u64).product())
    } else {
        None
    }
}

enum PointType {
    Symbol,
    Number((usize, usize), u32),
    Empty,
}

fn part2(input: &str) -> u64 {
    let lines = input.lines();
    let schematic: Vec<Vec<PointType>> = lines
        .enumerate()
        .map(|(row_idx, line)| {
            let mut digit_location: Option<((usize, usize), u32)> = None;
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    if c.is_ascii_digit() {
                        if let Some((location, value)) = digit_location {
                            PointType::Number(location, value)
                        } else {
                            let (_, num) = get_number(&line[col_idx..]).expect("no number");
                            digit_location = Some(((row_idx, col_idx), num));
                            PointType::Number((row_idx, col_idx), num)
                        }
                    } else if is_symbol(c) {
                        digit_location = None;
                        return PointType::Symbol;
                    } else {
                        digit_location = None;
                        return PointType::Empty;
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    schematic
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, cell)| match cell {
                    PointType::Symbol => get_gear_ratio(&schematic, (i, j)),
                    _ => None,
                })
                .sum::<u64>()
        })
        .sum()
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
