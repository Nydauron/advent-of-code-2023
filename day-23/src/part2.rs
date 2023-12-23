use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

pub fn part2(input: &str) -> usize {
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

    let mut stack = VecDeque::from([PreprocessingPath {
        curr_pos: start_pos,
        current_distance: 0,
        prev_junction_pos: start_pos,
    }]);
    let mut visited = HashSet::new();
    let mut junctions = BTreeMap::from([(
        start_pos,
        Junction {
            pos: start_pos,
            edges: vec![],
        },
    )]);

    while let Some(path) = stack.pop_back() {
        if visited.contains(&path.curr_pos) && !junctions.contains_key(&path.curr_pos)
            || (path.curr_pos == path.prev_junction_pos && path.current_distance == 2)
        {
            continue;
        }
        visited.insert(path.curr_pos);

        let directions = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        let (last_junction_pos, distance) = {
            let is_junction = path.curr_pos == dest_pos
                || (path.curr_pos != start_pos
                    && directions.iter().all(|direction| {
                        if let Some(next_pos) =
                            direction.calculate_offset_position(path.curr_pos, 1)
                        {
                            if let Some(next_tile) = map.get(&next_pos) {
                                return matches!(next_tile, Tile::SingleDirection(_));
                            }
                        }
                        true
                    }));
            if is_junction {
                let prev_pos = {
                    let prev_junction = junctions
                        .get_mut(&path.prev_junction_pos)
                        .expect("Previous junction id pointed to nothing");
                    prev_junction.edges.push(Edge {
                        cost: path.current_distance,
                        to: path.curr_pos,
                    });
                    prev_junction.pos
                };
                junctions
                    .entry(path.curr_pos)
                    .and_modify(|junction| {
                        junction.edges.push(Edge {
                            cost: path.current_distance,
                            to: prev_pos,
                        })
                    })
                    .or_insert(Junction {
                        pos: path.curr_pos,
                        edges: vec![Edge {
                            cost: path.current_distance,
                            to: prev_pos,
                        }],
                    });
                (path.curr_pos, 1)
            } else {
                (path.prev_junction_pos, path.current_distance + 1)
            }
        };

        directions.iter().for_each(|direction| {
            if let Some(next_pos) = direction.calculate_offset_position(path.curr_pos, 1) {
                if map.contains_key(&next_pos) {
                    stack.push_back(PreprocessingPath {
                        curr_pos: next_pos,
                        current_distance: distance,
                        prev_junction_pos: last_junction_pos,
                    });
                }
            }
        })
    }
    drop(stack);
    drop(visited);

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
        let node = junctions
            .get(&path.curr_pos)
            .expect("Could not fetch junction");
        path.visited_nodes.insert(node.pos);
        node.edges.iter().for_each(|edge| {
            if !path.visited_nodes.contains(&edge.to) {
                queue.push(HikingPath {
                    curr_pos: edge.to,
                    current_distance: path.current_distance + edge.cost,
                    visited_nodes: path.visited_nodes.clone(),
                })
            }
        });
    }

    distances.iter().max().cloned().unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreprocessingPath {
    curr_pos: (usize, usize),
    current_distance: usize,
    prev_junction_pos: (usize, usize),
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

#[derive(Debug, Clone)]
struct Junction {
    pos: (usize, usize),
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    cost: usize,
    to: (usize, usize),
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

        assert_eq!(part2(input), 154);
    }
}
