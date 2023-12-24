use itertools::Itertools;
use nalgebra::{Matrix1x6, Matrix3, Matrix3x1, Matrix6, Matrix6x1};
use nom::{
    bytes::complete::tag,
    character::complete::{self, space0},
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};

pub fn part2(input: &str) -> u64 {
    let rock_trajectory =
        part2_get_rock_trajectory(input).expect("Rock trajectory could not be found");
    (rock_trajectory.initial_pos.0 + rock_trajectory.initial_pos.1 + rock_trajectory.initial_pos.2)
        .round() as u64
}

fn get_cross_product_skew_symmetric_matrix(vector: &Matrix3x1<f64>) -> Matrix3<f64> {
    let [x, y, z] = vector.data.0[0];
    Matrix3::new(0_f64, -z, y, z, 0_f64, -x, -y, x, 0_f64)
}

fn part2_get_rock_trajectory(input: &str) -> Result<Trajectory, &str> {
    let hailstones = input
        .lines()
        .take(3)
        .map(|line| {
            let (_, hailstone) =
                parse_hailstone(line).expect("Line did not parse as a valid hailstone");
            hailstone
        })
        .collect_vec();

    let (matrix_a, vector_b) = hailstones.iter().combinations(2).take(2).enumerate().fold(
        (Matrix6::default(), Matrix6x1::default()),
        |(mut matrix_a, mut vector_b), (idx, hailstones)| {
            const DIMS: usize = 3;
            let a = hailstones[0];
            let b = hailstones[1];

            let a_pos_vector = Matrix3x1::new(a.initial_pos.0, a.initial_pos.1, a.initial_pos.2);
            let a_vel_vector = Matrix3x1::new(a.initial_vel.0, a.initial_vel.1, a.initial_vel.2);
            let b_pos_vector = Matrix3x1::new(b.initial_pos.0, b.initial_pos.1, b.initial_pos.2);
            let b_vel_vector = Matrix3x1::new(b.initial_vel.0, b.initial_vel.1, b.initial_vel.2);

            let cross_a = a_pos_vector.cross(&a_vel_vector);
            let cross_b = b_pos_vector.cross(&b_vel_vector);

            let partial_vector_b = cross_a - cross_b;
            for (row, &b) in partial_vector_b.iter().enumerate() {
                vector_b.fill_row(DIMS * idx + row, b);
            }

            let skew_symmetric_a_pos = get_cross_product_skew_symmetric_matrix(&a_pos_vector);
            let skew_symmetric_a_vel = get_cross_product_skew_symmetric_matrix(&a_vel_vector);
            let skew_symmetric_b_pos = get_cross_product_skew_symmetric_matrix(&b_pos_vector);
            let skew_symmetric_b_vel = get_cross_product_skew_symmetric_matrix(&b_vel_vector);

            let vel_matrix = skew_symmetric_a_vel - skew_symmetric_b_vel;
            let pos_matrix = -skew_symmetric_a_pos + skew_symmetric_b_pos;

            for row_idx in 0..DIMS {
                let [px, py, pz] = pos_matrix.data.0[row_idx];
                let [vx, vy, vz] = vel_matrix.data.0[row_idx];
                let row = Matrix1x6::new(vx, vy, vz, px, py, pz);
                matrix_a.set_row(DIMS * idx + row_idx, &row);
            }
            (matrix_a, vector_b)
        },
    );

    let inverse_a = match matrix_a.try_inverse() {
        Some(matrix) => matrix,
        None => return Err("Could not get inverse"),
    };

    let rock_pos_and_vel = inverse_a * vector_b;

    let [px, py, pz, vx, vy, vz] = rock_pos_and_vel.data.0[0];
    Ok(Trajectory {
        initial_pos: (px.round(), py.round(), pz.round()),
        initial_vel: (vx.round(), vy.round(), vz.round()),
    })
}

#[derive(Debug, Clone, Copy)]
struct Trajectory {
    initial_pos: (f64, f64, f64),
    initial_vel: (f64, f64, f64),
}

impl PartialEq for Trajectory {
    fn eq(&self, other: &Self) -> bool {
        self.initial_pos.0 == other.initial_pos.0
            && self.initial_pos.1 == other.initial_pos.1
            && self.initial_pos.2 == other.initial_pos.2
            && self.initial_vel.0 == other.initial_vel.0
            && self.initial_vel.1 == other.initial_vel.1
            && self.initial_vel.2 == other.initial_vel.2
    }
}

impl Eq for Trajectory {}

fn parse_hailstone(input: &str) -> IResult<&str, Trajectory> {
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
        Trajectory {
            initial_pos: pos,
            initial_vel: vel,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

        assert_eq!(
            part2_get_rock_trajectory(input),
            Ok(Trajectory {
                initial_pos: (24_f64, 13_f64, 10_f64),
                initial_vel: (-3_f64, 1_f64, 2_f64)
            })
        );
    }
}
