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

fn search_for_symbol(
    schematic: &Vec<Vec<(u32, bool)>>,
    number_char_len: usize,
    number_location: (usize, usize),
) -> bool {
    for row in schematic
        .iter()
        .take((schematic.len() - 1).min(number_location.0 + 1))
        .skip(0.max(number_location.0 as i64 - 1) as usize)
    {
        for cell in row
            .iter()
            .take((row.len() - 1).min(number_location.1 + number_char_len))
            .skip(0.max(number_location.1 as i32 - 1) as usize)
        {
            if cell.1 {
                return true;
            }
        }
    }
    false
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let schematic = lines
        .map(|line| {
            let mut digit_slice: Option<u32> = None;
            line.chars()
                .enumerate()
                .map(|(idx, c)| {
                    if c.is_ascii_digit() {
                        if digit_slice.is_none() {
                            let (_, num) = get_number(&line[idx..]).expect("no number");
                            digit_slice = Some(num);
                            return (num, false);
                        }
                        (0, false)
                    } else if is_symbol(c) {
                        digit_slice = None;
                        return (0, true);
                    } else {
                        digit_slice = None;
                        return (0, false);
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, row) in schematic.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell.0 > 0 {
                let num_char_len = ((cell.0 as f32).log10().floor() as u32) + 1;
                if search_for_symbol(&schematic, num_char_len as usize, (i, j)) {
                    sum += cell.0;
                }
            }
        }
    }

    sum
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
