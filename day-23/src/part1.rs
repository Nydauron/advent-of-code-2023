use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

pub fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col_idx, c)| match c {
                    '#' => None,
                    '.' => Some(((row_idx, col_idx), Tile::Open)),
                    '^' => Some(((row_idx, col_idx), Tile::SingleDirection(Direction::North))),
                    'v' => Some(((row_idx, col_idx), Tile::SingleDirection(Direction::South))),
                    '>' => Some(((row_idx, col_idx), Tile::SingleDirection(Direction::East))),
                    '<' => Some(((row_idx, col_idx), Tile::SingleDirection(Direction::West))),
                    c => panic!("Encountered unexpected character: {}", c),
                })
        })
        .collect::<BTreeMap<_, _>>();

    let start_pos = (0, 1);
    let dest_pos = map.keys().max().cloned().unwrap();

    let mut queue = BinaryHeap::from([HikingPath {
        curr_pos: start_pos,
        current_distance: 0,
        visited_nodes: HashSet::new(),
    }]);
    let mut distances = vec![];

    while let Some(mut path) = queue.pop() {
        if path.curr_pos == dest_pos {
            distances.push(path.current_distance);
            continue;
        }
        path.visited_nodes.insert(path.curr_pos);

        if let Some(path_type) = map.get(&path.curr_pos) {
            match path_type {
                Tile::Open => {
                    // attempt to move in every direction
                    let directions = [
                        Direction::North,
                        Direction::South,
                        Direction::East,
                        Direction::West,
                    ];
                    directions.iter().for_each(|direction| {
                        if let Some(next_pos) =
                            direction.calculate_offset_position(path.curr_pos, 1)
                        {
                            if !path.visited_nodes.contains(&next_pos) {
                                queue.push(HikingPath {
                                    curr_pos: next_pos,
                                    current_distance: path.current_distance + 1,
                                    visited_nodes: path.visited_nodes.clone(),
                                })
                            }
                        }
                    });
                }
                Tile::SingleDirection(direction) => {
                    if let Some(next_pos) = direction.calculate_offset_position(path.curr_pos, 1) {
                        if !path.visited_nodes.contains(&next_pos) {
                            queue.push(HikingPath {
                                curr_pos: next_pos,
                                current_distance: path.current_distance + 1,
                                visited_nodes: path.visited_nodes.clone(),
                            })
                        }
                    }
                }
            }
        }
    }

    distances.iter().max().cloned().unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HikingPath {
    curr_pos: (usize, usize),
    current_distance: usize,
    visited_nodes: HashSet<(usize, usize)>,
}

impl Ord for HikingPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.current_distance.cmp(&other.current_distance)
    }
}

impl PartialOrd for HikingPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
enum Tile {
    Open,
    SingleDirection(Direction),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

        assert_eq!(part1(input), 94);
    }
}
