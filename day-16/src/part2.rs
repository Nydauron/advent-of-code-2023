use std::collections::{BTreeMap, HashSet, VecDeque};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn part2(input: &str) -> usize {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line.chars().enumerate().map(move |(col_idx, c)| {
                let pos = (row_idx as i64, col_idx as i64);
                let tile_type = match c {
                    '.' => TileType::Empty,
                    '-' => TileType::HorizontalSplitter,
                    '|' => TileType::VerticalSplitter,
                    '/' => TileType::PositiveMirror,
                    '\\' => TileType::NegativeMirror,
                    c => unreachable!("Input provided has illegal char: {}", c),
                };
                (
                    pos,
                    Tile {
                        tile_type,
                        incoming_light: HashSet::new(),
                    },
                )
            })
        })
        .collect::<BTreeMap<_, _>>();

    let max_rows = input.lines().count() as i64;
    let max_cols = input.lines().next().unwrap().len() as i64;

    let mut starting_points = vec![];

    for r in 0..max_rows {
        starting_points.push(((r, 0), Direction::East));
    }
    for r in 0..max_rows {
        starting_points.push(((r, max_cols - 1), Direction::West));
    }
    for c in 1..(max_cols - 1) {
        starting_points.push(((0, c), Direction::South));
    }
    for c in 1..(max_cols - 1) {
        starting_points.push(((max_rows - 1, c), Direction::North));
    }

    starting_points
        .par_iter()
        .map(|start| get_all_energized_tiles(map.clone(), *start))
        .max()
        .unwrap()
}

fn get_all_energized_tiles(
    mut map: BTreeMap<(i64, i64), Tile>,
    start: ((i64, i64), Direction),
) -> usize {
    let mut queue = VecDeque::from([start]);

    while let Some((pos, incoming_direction)) = queue.pop_front() {
        if let Some(tile) = map.get_mut(&pos) {
            if tile.incoming_light.contains(&incoming_direction) {
                // path has already been evaluated, skip
                continue;
            }
            tile.incoming_light.insert(incoming_direction);
            match tile.tile_type {
                TileType::Empty => {
                    let offset = incoming_direction.get_offset();
                    queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), incoming_direction));
                }
                TileType::HorizontalSplitter => match incoming_direction {
                    Direction::North | Direction::South => {
                        let next_directions = [Direction::East, Direction::West];
                        for direction in next_directions {
                            let offset = direction.get_offset();
                            queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), direction));
                        }
                    }
                    direction => {
                        let offset = direction.get_offset();
                        queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), direction));
                    }
                },
                TileType::VerticalSplitter => match incoming_direction {
                    Direction::East | Direction::West => {
                        let next_directions = [Direction::North, Direction::South];
                        for direction in next_directions {
                            let offset = direction.get_offset();
                            queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), direction));
                        }
                    }
                    direction => {
                        let offset = direction.get_offset();
                        queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), direction));
                    }
                },
                TileType::PositiveMirror => match incoming_direction {
                    Direction::North => {
                        let next_direction = Direction::East;
                        let offset = next_direction.get_offset();
                        queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), next_direction));
                    }
                    Direction::South => {
                        let next_direction = Direction::West;
                        let offset = next_direction.get_offset();
                        queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), next_direction));
                    }
                    Direction::East => {
                        let next_direction = Direction::North;
                        let offset = next_direction.get_offset();
                        queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), next_direction));
                    }
                    Direction::West => {
                        let next_direction = Direction::South;
                        let offset = next_direction.get_offset();
                        queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), next_direction));
                    }
                },
                TileType::NegativeMirror => match incoming_direction {
                    Direction::North => {
                        let next_direction = Direction::West;
                        let offset = next_direction.get_offset();
                        queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), next_direction));
                    }
                    Direction::South => {
                        let next_direction = Direction::East;
                        let offset = next_direction.get_offset();
                        queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), next_direction));
                    }
                    Direction::East => {
                        let next_direction = Direction::South;
                        let offset = next_direction.get_offset();
                        queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), next_direction));
                    }
                    Direction::West => {
                        let next_direction = Direction::North;
                        let offset = next_direction.get_offset();
                        queue.push_back(((pos.0 + offset.0, pos.1 + offset.1), next_direction));
                    }
                },
            }
        }
    }

    map.iter()
        .filter(|(_, tile)| tile.incoming_light.len() > 0)
        .count()
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Empty,
    HorizontalSplitter,
    VerticalSplitter,
    PositiveMirror,
    NegativeMirror,
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
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    incoming_light: HashSet<Direction>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

        assert_eq!(part2(input), 51);
    }
}
