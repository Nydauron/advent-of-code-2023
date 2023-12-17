use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashSet},
};

pub fn part1(input: &str) -> u64 {
    let mut map = input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line.chars().enumerate().map(move |(col_idx, c)| {
                (
                    (row_idx as i64, col_idx as i64),
                    Tile {
                        cost: c as u8 - b'0',
                        visited_with: HashSet::new(),
                    },
                )
            })
        })
        .collect::<BTreeMap<_, _>>();

    let dest = map.keys().max().cloned().unwrap();
    let source = (0_i64, 0_i64);

    let mut queue = BinaryHeap::from([Reverse(NodeCost {
        pos: source,
        cost: -1 * (map.get(&source).unwrap().cost as i64),
        been_straight_for: 0,
        incoming_direction: Direction::East,
    })]);

    let mut total_cost = None;
    while let Some(Reverse(curr)) = queue.pop() {
        if let Some(tile) = map.get_mut(&curr.pos) {
            if curr.pos == dest {
                total_cost = Some(curr.cost + tile.cost as i64);
                break;
            }
            if tile
                .visited_with
                .contains(&(curr.been_straight_for, curr.incoming_direction))
            {
                // we have already evaluated this path, skip
                continue;
            }

            tile.visited_with
                .insert((curr.been_straight_for, curr.incoming_direction));
            let directions = [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ];
            let max_straight_length = 3;
            for &d in directions.iter().filter(|&d| {
                *d != curr.incoming_direction.opposite_direction()
                    && (*d != curr.incoming_direction
                        || curr.been_straight_for < max_straight_length)
            }) {
                let offset = d.get_offset();
                let straight_length = if d == curr.incoming_direction {
                    curr.been_straight_for + 1
                } else {
                    1
                };
                queue.push(Reverse(NodeCost {
                    pos: (curr.pos.0 + offset.0, curr.pos.1 + offset.1),
                    incoming_direction: d,
                    been_straight_for: straight_length,
                    cost: curr.cost + tile.cost as i64,
                }))
            }
        }
    }
    total_cost.unwrap().max(0) as u64
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    cost: u8,
    visited_with: HashSet<(u8, Direction)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_offset(&self) -> (i64, i64) {
        match &self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
        }
    }

    fn opposite_direction(&self) -> Direction {
        match &self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NodeCost {
    pos: (i64, i64),
    cost: i64,
    been_straight_for: u8,
    incoming_direction: Direction,
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        assert_eq!(part1(input), 102);
    }
}
