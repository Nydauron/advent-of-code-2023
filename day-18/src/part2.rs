use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{self, alpha1, space1},
    combinator::map_res,
    sequence::{preceded, tuple},
    IResult,
};

pub fn part2(input: &str) -> i64 {
    let starting_pos = (0, 0);
    let (map, _) = input
        .lines()
        .map(|line| {
            let (_, step) = parse_line(line).expect("Line did not parse correctly");
            step
        })
        .fold(
            (vec![Boundary { pos: starting_pos }], starting_pos),
            |(mut acc, curr_pos), step| {
                let next_pos = match step.direction {
                    Direction::Up(length) => (curr_pos.0 - length as i64, curr_pos.1),
                    Direction::Down(length) => (curr_pos.0 + length as i64, curr_pos.1),
                    Direction::Left(length) => (curr_pos.0, curr_pos.1 - length as i64),
                    Direction::Right(length) => (curr_pos.0, curr_pos.1 + length as i64),
                };

                acc.push(Boundary { pos: next_pos });
                (acc, next_pos)
            },
        );

    let area = map
        .iter()
        .map_windows(|[vertice_i, vertice_j]| {
            vertice_i.pos.0 * vertice_j.pos.1 - vertice_i.pos.1 * vertice_j.pos.0
        })
        .sum::<i64>()
        .abs()
        / 2;
    let perimeter = map
        .iter()
        .map_windows(|[vertice_i, vertice_j]| {
            (vertice_i.pos.0 - vertice_j.pos.0).abs() + (vertice_i.pos.1 - vertice_j.pos.1).abs()
        })
        .sum::<i64>();

    area + perimeter / 2 + 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Boundary {
    pos: (i64, i64),
}

impl Ord for Boundary {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialOrd for Boundary {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DiagonalOrder((u64, u64));

impl Ord for DiagonalOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 .0 + self.0 .1).cmp(&(other.0 .0 + other.0 .1))
    }
}
impl PartialOrd for DiagonalOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up(u64),
    Down(u64),
    Left(u64),
    Right(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DigStep {
    direction: Direction,
}

fn from_hex(input: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str, length: usize) -> IResult<&str, u64> {
    map_res(take_while_m_n(length, length, is_hex_digit), from_hex)(input)
}

// modified version of the hex color parser from the nom docs
fn hex_color(input: &str) -> IResult<&str, DigStep> {
    let (input, _) = tag("#")(input)?;
    let (input, amount) = hex_primary(input, 5)?;
    let (input, direction) = hex_primary(input, 1)?;

    let direction = match direction {
        0 => Direction::Right(amount),
        1 => Direction::Down(amount),
        2 => Direction::Left(amount),
        3 => Direction::Up(amount),
        val => panic!("unknown direction value found: {}", val),
    };
    Ok((input, DigStep { direction }))
}

fn parse_line(input: &str) -> IResult<&str, DigStep> {
    let (input, (_, dig_stap, _)) = preceded(
        tuple((alpha1, space1, complete::u64, space1)),
        tuple((tag("("), hex_color, tag(")"))),
    )(input)?;

    IResult::Ok((input, dig_stap))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        assert_eq!(part2(input), 952408144115);
    }
}
