use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashSet},
};

pub fn part2(input: &str) -> u64 {
    let cost_map = input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(move |(col_idx, c)| ((row_idx as i64, col_idx as i64), c as u8 - b'0'))
        })
        .collect::<BTreeMap<_, _>>();

    let dest = cost_map.keys().max().cloned().unwrap();
    let source = (0_i64, 0_i64);

    const MIN_STRAIGHT_LENGTH: i64 = 4;
    const MAX_STRAIGHT_LENGTH: i64 = 10;

    let mut queue = BinaryHeap::new();
    let starting_directions = [Direction::South, Direction::East];
    for d in starting_directions {
        let mut running_cost = 0;
        for straight_for in 1..=MAX_STRAIGHT_LENGTH {
            let offset = d.get_offset(straight_for);
            let next_pos = (source.0 + offset.0, source.1 + offset.1);
            if let Some(&next_tile_cost) = cost_map.get(&next_pos) {
                running_cost += next_tile_cost as i64;
                if straight_for < MIN_STRAIGHT_LENGTH {
                    continue;
                }
                let heuristic = manhattan_distance(next_pos, dest);
                queue.push(Reverse(NodeCost {
                    pos: next_pos,
                    incoming_direction: d,
                    cost: running_cost,
                    cost_with_heuristic: running_cost + heuristic,
                }));
            } else {
                break;
            }
        }
    }

    let mut visited = HashSet::new();
    let mut total_cost = None;
    while let Some(Reverse(curr)) = queue.pop() {
        if curr.pos == dest {
            total_cost = Some(curr.cost as i64);
            break;
        }
        if visited.contains(&(curr.pos, curr.incoming_direction)) {
            // we have already evaluated this path, skip
            continue;
        }

        visited.insert((curr.pos, curr.incoming_direction));
        let directions = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        for &d in directions.iter().filter(|&&d| {
            d != curr.incoming_direction && d != curr.incoming_direction.opposite_direction()
        }) {
            let mut running_cost = 0;
            for straight_for in 1..=MAX_STRAIGHT_LENGTH {
                let offset = d.get_offset(straight_for);
                let base_cost = curr.cost as i64;
                let next_pos = (curr.pos.0 + offset.0, curr.pos.1 + offset.1);
                if let Some(&next_tile_cost) = cost_map.get(&next_pos) {
                    running_cost += next_tile_cost as i64;
                    if straight_for < MIN_STRAIGHT_LENGTH {
                        continue;
                    }
                    let heuristic = manhattan_distance(next_pos, dest);
                    queue.push(Reverse(NodeCost {
                        pos: next_pos,
                        incoming_direction: d,
                        cost: base_cost + running_cost,
                        cost_with_heuristic: base_cost + running_cost + heuristic,
                    }));
                } else {
                    break;
                }
            }
        }
    }
    total_cost.unwrap().max(0) as u64
}

fn manhattan_distance(src: (i64, i64), dest: (i64, i64)) -> i64 {
    (src.0 - dest.0).abs() + (src.1 - dest.1).abs()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    cost: u8,
    visited_with: HashSet<Direction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_offset(&self, scale: i64) -> (i64, i64) {
        match &self {
            Self::North => (-1 * scale, 0),
            Self::South => (1 * scale, 0),
            Self::East => (0, 1 * scale),
            Self::West => (0, -1 * scale),
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
    cost_with_heuristic: i64,
    incoming_direction: Direction,
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost_with_heuristic.cmp(&other.cost_with_heuristic)
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
    fn test_part2() {
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

        assert_eq!(part2(input), 94);
    }

    #[test]
    fn test_simple_part2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";

        assert_eq!(part2(input), 71);
    }
}
