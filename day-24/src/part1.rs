use std::ops::RangeInclusive;

use itertools::Itertools;
use nalgebra::{Matrix2, Matrix2x1};
use nom::{
    bytes::complete::tag,
    character::complete::{self, space0},
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};

pub fn part1(input: &str) -> u64 {
    part1_with_bounds(input, 200000000000000f64..=400000000000000f64)
}

fn part1_with_bounds(input: &str, bounds: RangeInclusive<f64>) -> u64 {
    let hailstones = input
        .lines()
        .map(|line| {
            let (_, hailstone) =
                parse_hailstone(line).expect("Line did not parse as a valid hailstone");
            hailstone
        })
        .collect_vec();

    hailstones
        .iter()
        .combinations(2)
        .filter_map(|hailstones| {
            let a = hailstones[0];
            let b = hailstones[1];
            // ax + avxt = X
            // ay + avyt = Y
            // t = (X - ax) / avx
            // ay + (avy)/(avx)(X - ax) = Y
            // avy*X - avx*Y = -ay * avx + avy * ax

            let matrix_a = Matrix2::new(
                a.initial_vel.1,
                -a.initial_vel.0,
                b.initial_vel.1,
                -b.initial_vel.0,
            );
            let matrix_inverse_a = matrix_a.try_inverse()?;
            let vector_b = Matrix2x1::new(
                a.initial_pos.0 * a.initial_vel.1 - a.initial_pos.1 * a.initial_vel.0,
                b.initial_pos.0 * b.initial_vel.1 - b.initial_pos.1 * b.initial_vel.0,
            );

            let cross = matrix_inverse_a * vector_b;
            // if t is negative in either parmetrization, we disgard it (since negative t values
            // represent the past)
            let t_a = (cross.x - a.initial_pos.0) / a.initial_vel.0;
            let t_b = (cross.x - b.initial_pos.0) / b.initial_vel.0;
            (t_a >= 0_f64 && t_b >= 0_f64).then_some((cross.x, cross.y))
        })
        .filter(|collision_pos| {
            bounds.contains(&collision_pos.0) && bounds.contains(&collision_pos.1)
        })
        .count()
        .try_into()
        .unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    initial_pos: (f64, f64, f64),
    initial_vel: (f64, f64, f64),
}

fn parse_hailstone(input: &str) -> IResult<&str, Hailstone> {
    let pos_vector = tuple((
        complete::i64,
        preceded(tag(","), space0),
        complete::i64,
        preceded(tag(","), space0),
        complete::i64,
    ))
    .map(|(x, _, y, _, z)| (x as f64, y as f64, z as f64));
    let vel_vector = tuple((
        complete::i64,
        preceded(tag(","), space0),
        complete::i64,
        preceded(tag(","), space0),
        complete::i64,
    ))
    .map(|(x, _, y, _, z)| (x as f64, y as f64, z as f64));
    let (input, (pos, vel)) =
        separated_pair(pos_vector, tuple((space0, tag("@"), space0)), vel_vector)(input)?;

    Ok((
        input,
        Hailstone {
            initial_pos: pos,
            initial_vel: vel,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

        assert_eq!(part1_with_bounds(input, 7_f64..=27_f64), 2);
    }
}
