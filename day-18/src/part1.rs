use nom::{
    character::complete::{self, alpha1, space1},
    sequence::tuple,
    IResult,
};

pub fn part1(input: &str) -> i64 {
    const STARTING_POS: (i64, i64) = (0, 0);
    let (map, _) = input
        .lines()
        .map(|line| {
            let (_, step) = parse_line(line).expect("Line did not parse correctly");
            step
        })
        .fold(
            (vec![Boundary { pos: STARTING_POS }], STARTING_POS),
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
enum Direction {
    Up(u64),
    Down(u64),
    Left(u64),
    Right(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DigStep {
    direction: Direction,
}

fn parse_line(input: &str) -> IResult<&str, DigStep> {
    let (input, (direction, _, amount, _)) = tuple((alpha1, space1, complete::u64, space1))(input)?;
    let direction = match direction {
        "R" => Direction::Right(amount),
        "L" => Direction::Left(amount),
        "U" => Direction::Up(amount),
        "D" => Direction::Down(amount),
        c => panic!("unknown character found: {}", c),
    };

    IResult::Ok((input, DigStep { direction }))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
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

        assert_eq!(part1(input), 62);
    }
}
