use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub fn part2(input: &str) -> u64 {
    let cost_map = input
        .chars()
        .filter_map(|c| {
            if c.is_numeric() {
                Some(c as u8 - b'0')
            } else if c == '\n' {
                None
            } else {
                panic!("Unknown character found: {}", c)
            }
        })
        .collect::<Vec<_>>();

    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = cost_map.len() / width;
    let dest = (height - 1, width - 1);
    let source = (0_usize, 0_usize);

    const MIN_STRAIGHT_LENGTH: usize = 4;
    const MAX_STRAIGHT_LENGTH: usize = 10;

    let mut queue = BinaryHeap::new();
    let starting_directions = [Direction::South, Direction::East];
    for d in starting_directions {
        let mut running_cost = 0;
        for straight_for in 1..=MAX_STRAIGHT_LENGTH {
            if let Some(next_pos) = d.calculate_offset_position(source, straight_for) {
                if let Some(&next_tile_cost) = cost_map.get(pos_to_index(next_pos, width)) {
                    running_cost += next_tile_cost as u64;
                    if straight_for < MIN_STRAIGHT_LENGTH {
                        continue;
                    }
                    let heuristic = manhattan_distance(next_pos, dest) as u64;
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
    }

    let mut visited = HashSet::new();
    let mut total_cost = None;
    while let Some(Reverse(curr)) = queue.pop() {
        if curr.pos == dest {
            total_cost = Some(curr.cost as i64);
            break;
        }
        let index = pos_to_index(curr.pos, width);
        if visited.contains(&(index, curr.incoming_direction)) {
            // we have already evaluated this path, skip
            continue;
        }

        visited.insert((index, curr.incoming_direction));
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
                let base_cost = curr.cost;
                if let Some(next_pos) = d.calculate_offset_position(curr.pos, straight_for) {
                    if let Some(&next_tile_cost) = cost_map.get(pos_to_index(next_pos, width)) {
                        running_cost += next_tile_cost as u64;
                        if straight_for < MIN_STRAIGHT_LENGTH {
                            continue;
                        }
                        let heuristic = manhattan_distance(next_pos, dest) as u64;
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
    }
    total_cost.unwrap().max(0) as u64
}

fn pos_to_index(pos: (usize, usize), width: usize) -> usize {
    pos.0 * width + pos.1
}

fn manhattan_distance(src: (usize, usize), dest: (usize, usize)) -> usize {
    let difference_tuples = [(src.0, dest.0), (src.1, dest.1)];
    difference_tuples
        .into_iter()
        .map(|(src, dest)| {
            (src > dest)
                .then(|| src - dest)
                .unwrap_or_else(|| dest - src)
        })
        .sum()
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
    fn calculate_offset_position(
        &self,
        pos: (usize, usize),
        offset_by: usize,
    ) -> Option<(usize, usize)> {
        match &self {
            Self::North => (offset_by <= pos.0).then(|| (pos.0 - offset_by, pos.1)),
            Self::South => Some((pos.0 + offset_by, pos.1)),
            Self::East => Some((pos.0, pos.1 + offset_by)),
            Self::West => (offset_by <= pos.1).then(|| (pos.0, pos.1 - offset_by)),
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
    pos: (usize, usize),
    cost: u64,
    cost_with_heuristic: u64,
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
