use std::{
    collections::{BTreeSet, HashMap},
    ops::RangeInclusive,
};

use itertools::Itertools;
use nom::{
    bytes::complete::{is_not, tag},
    multi::separated_list1,
    IResult,
};

type BrickId = usize;

pub fn part1(input: &str) -> usize {
    let bricks = input
        .lines()
        .map(|line| {
            let (_, brick) = parse_brick(line).expect("Line did not parse to valid brick");
            brick
        })
        .collect::<BTreeSet<_>>();

    let mut stationary_bricks = vec![];
    let mut current_heights = HashMap::<(usize, usize), (usize, BrickId)>::new();
    bricks.iter().for_each(|brick| {
        let top_staionary_bricks = brick
            .x_range
            .clone()
            .cartesian_product(brick.y_range.clone())
            .filter_map(|xy_pos| current_heights.get(&xy_pos).map(|&a| a))
            .max_set_by(|a, b| a.0.cmp(&b.0));
        let z = top_staionary_bricks
            .get(0)
            .map(|(height, _)| *height)
            .unwrap_or(1);
        let new_stationary_brick = StationaryBrick {
            bricks_directly_above: vec![],
            support_count: top_staionary_bricks
                .iter()
                .map(|(_, brick_id)| *brick_id)
                .unique()
                .count(),
        };

        let new_z = z + brick.height;
        stationary_bricks.push(new_stationary_brick);
        let brick_id = (stationary_bricks.len() - 1) as BrickId;
        for (_, supporting_brick_id) in top_staionary_bricks {
            stationary_bricks
                .get_mut(supporting_brick_id as usize)
                .unwrap()
                .bricks_directly_above
                .push(brick_id);
        }
        brick
            .x_range
            .clone()
            .cartesian_product(brick.y_range.clone())
            .for_each(|xy_pos| {
                current_heights
                    .entry(xy_pos)
                    .and_modify(|(height, stationary_brick_ptr)| {
                        *height = new_z;
                        *stationary_brick_ptr = brick_id;
                    })
                    .or_insert((new_z, brick_id));
            });
    });

    stationary_bricks
        .iter()
        .filter(|brick| {
            // If we were to remove `brick` ...
            //
            // Do any of the driectly above bricks fall? (They will fall if support count == 1)
            for brick_id in brick.bricks_directly_above.iter() {
                if let Some(above_brick) = stationary_bricks.get(*brick_id) {
                    if above_brick.support_count == 1 {
                        return false;
                    }
                }
            }
            true
        })
        .count()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FallingBrick {
    ends: [(usize, usize, usize); 2],
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
    top_most_z: usize,
    height: usize,
}

#[derive(Debug, Clone)]
struct StationaryBrick {
    bricks_directly_above: Vec<BrickId>,
    // bricks_directly_below: Vec<&'a StationaryBrick<'a>>,
    support_count: usize,
}

impl Ord for FallingBrick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let min_z = self.ends.iter().min_by(|a, b| a.2.cmp(&b.2)).unwrap();
        let other_min_z = other.ends.iter().min_by(|a, b| a.2.cmp(&b.2)).unwrap();
        min_z.2.cmp(&other_min_z.2)
    }
}

impl PartialOrd for FallingBrick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn parse_brick(input: &str) -> IResult<&str, FallingBrick> {
    let (input, start) = separated_list1(tag(","), is_not("~,"))(input)?;
    let (input, _) = tag("~")(input)?;
    let (input, end) = separated_list1(tag(","), is_not("~,"))(input)?;
    let start = start
        .iter()
        .map(|coord| coord.parse::<usize>().expect("could not parse number"))
        .tuples::<(_, _, _)>()
        .exactly_one()
        .expect("not exactly one");
    let end = end
        .iter()
        .map(|coord| coord.parse::<usize>().expect("could not parse number"))
        .tuples::<(_, _, _)>()
        .exactly_one()
        .expect("not exactly one");
    let x_range = if start.0 < end.0 {
        start.0..=end.0
    } else {
        end.0..=start.0
    };
    let y_range = if start.1 < end.1 {
        start.1..=end.1
    } else {
        end.1..=start.1
    };
    let top_most_z = start.2.max(end.2);
    let height = if start.2 < end.2 {
        end.2 - start.2 + 1
    } else {
        start.2 - end.2 + 1
    };
    Ok((
        input,
        FallingBrick {
            ends: [start, end],
            x_range,
            y_range,
            top_most_z,
            height,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        assert_eq!(part1(input), 5);
    }
}
