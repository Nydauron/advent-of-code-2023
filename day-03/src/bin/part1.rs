use std::collections::HashMap;

use nom::{character, AsChar, IResult};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("input.txt");
    println!("{}", part1(input))
}

fn get_number(input: &str) -> IResult<&str, u32> {
    let (input, number) = character::complete::u32(input)?;
    Ok((input, number))
}

fn is_symbol(c: char) -> bool {
    !c.is_alphanum() && c != '.'
}

#[derive(Debug)]
enum PointType {
    Symbol,
    Number((usize, usize), u32),
    Empty,
}

fn numbers_near_symbol(
    schematic: &Vec<Vec<PointType>>,
    symbol_location: (usize, usize),
) -> Vec<u32> {
    let mut numbers = HashMap::new();
    for row in schematic
        .iter()
        .take(symbol_location.0 + 2)
        .skip(0.max(symbol_location.0 as i64 - 1) as usize)
    {
        for cell in row
            .iter()
            .take(symbol_location.1 + 2)
            .skip(0.max(symbol_location.1 as i64 - 1) as usize)
        {
            if let PointType::Number(pos, value) = cell {
                numbers.entry(*pos).or_insert(*value);
            }
        }
    }
    let return_value = numbers.values().cloned().collect::<Vec<_>>();
    return_value
}

fn part1(input: &str) -> u32 {
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
                .map(|(j, cell)| match cell {
                    PointType::Symbol => numbers_near_symbol(&schematic, (i, j)),
                    _ => vec![],
                })
                .fold(Vec::new(), |mut acc, new_nums| {
                    acc.extend(new_nums);
                    acc
                })
                .iter()
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
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
        assert_eq!(part1(input), 4361);
    }
    #[test]
    fn test_part1_custom() {
        let input = "..76...968............108...@.....556.....................=..........*...............412..313...575......../...........................*107.
............773/..891............*....................744.....805...14................../..../................320&.567..#...................
.962..708............&........399....146.....385.................*..........825.......................................-..655....485...-.....
...*.........+..........................*76...+..................242....997..*......185..........207.390..870...883............*.......337..";
        assert_eq!(part1(input), 9270);
    }
}
