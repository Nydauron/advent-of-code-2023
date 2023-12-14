use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ops::Range,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Sphere,
    Cube,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VerticalBin {
    column: usize,
    row_range: Range<usize>,
}

impl Ord for VerticalBin {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.column == other.column {
            self.row_range.start.cmp(&other.row_range.start)
        } else {
            self.column.cmp(&other.column)
        }
    }
}

impl PartialOrd for VerticalBin {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn part2(input: &str) -> usize {
    let platform = input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col_idx, c)| match c {
                    'O' => Some(((row_idx, col_idx), Rock::Sphere)),
                    '#' => Some(((row_idx, col_idx), Rock::Cube)),
                    '.' => Some(((row_idx, col_idx), Rock::Empty)),
                    c => panic!("Unknown char found: {}", c),
                })
        })
        .collect::<BTreeMap<_, _>>();

    let max_row = input.lines().count();
    let max_col = input.lines().next().unwrap().len();
    let spheres = get_new_platform(platform, max_row, max_col);

    spheres.iter().map(|pos| max_row - pos.0).sum()
}

fn get_new_platform(
    mut platform: BTreeMap<(usize, usize), Rock>,
    max_row: usize,
    max_col: usize,
) -> BTreeSet<(usize, usize)> {
    let mut cycle_mem: HashMap<Vec<usize>, BTreeSet<(usize, usize)>> = HashMap::new();

    let cube_rocks = platform
        .iter()
        .filter(|(_, &rock)| rock == Rock::Cube)
        .map(|(pos, _)| pos)
        .cloned()
        .collect_vec();

    let cube_rocks_with_bottom_border = cube_rocks
        .iter()
        .cloned()
        .chain((0..max_col).map(|col| (max_row, col)));

    let vertical_bins = (0..max_col)
        .flat_map(|col| {
            let mut last_cube_rock_row: Option<usize> = None;
            cube_rocks_with_bottom_border
                .clone()
                .filter_map(move |(rock_row, rock_col)| {
                    if rock_col != col {
                        None
                    } else {
                        if let Some(cube_rock_row_idx) = last_cube_rock_row {
                            let row_range = (cube_rock_row_idx + 1)..rock_row;
                            last_cube_rock_row = Some(rock_row);
                            Some(VerticalBin {
                                column: col,
                                row_range,
                            })
                        } else {
                            last_cube_rock_row = Some(rock_row);
                            Some(VerticalBin {
                                column: col,
                                row_range: 0..rock_row,
                            })
                        }
                    }
                })
        })
        .collect::<BTreeSet<_>>();

    let mut spheres = platform
        .iter()
        .filter(|(_, &rock)| rock == Rock::Sphere)
        .map(|(pos, _)| pos)
        .cloned()
        .collect::<BTreeSet<_>>();

    let mut cycles_completed = 0;
    let mut cycle_length = None;
    let mut start_bin_counts = None;
    let total_cycles = 1_000_000_000_usize;
    for cycle in 0..total_cycles {
        let next_bin_counts = vertical_bins
            .iter()
            .map(|bin| {
                spheres
                    .iter()
                    .filter(|(row_idx, col_idx)| {
                        *col_idx == bin.column && bin.row_range.contains(row_idx)
                    })
                    .count()
            })
            .collect_vec();

        if let Some(cycle_result) = cycle_mem.get(&next_bin_counts) {
            if let Some((start_cycle_num, start_bin_counts)) = start_bin_counts.clone() {
                // check for cycle
                // if there is a cycle, then set cycles_completed and break
                if start_bin_counts == next_bin_counts {
                    // found cycle
                    cycles_completed = cycle;
                    cycle_length = Some(cycle - start_cycle_num);
                    break;
                }
            }
            // otherwise set spheres to cycle_result
            spheres = cycle_result.clone();
            continue;
        }

        platform = cube_rocks
            .iter()
            .map(|&pos| (pos, Rock::Cube))
            .chain(spheres.iter().map(|&pos| (pos, Rock::Sphere)))
            .collect::<BTreeMap<_, _>>();
        spheres = (0..max_col)
            .map(|col_idx| {
                let mut next_available_spot = 0;
                platform.iter().filter(|(pos, _)| pos.1 == col_idx).fold(
                    vec![],
                    |mut acc, ((row_idx, _), rock)| {
                        match rock {
                            Rock::Sphere => {
                                acc.push((next_available_spot, col_idx));
                                next_available_spot += 1;
                            }
                            Rock::Cube => {
                                next_available_spot = row_idx + 1;
                            }
                            Rock::Empty => {}
                        }
                        acc
                    },
                )
            })
            .flatten()
            .collect::<BTreeSet<_>>();

        platform = cube_rocks
            .iter()
            .map(|&pos| (pos, Rock::Cube))
            .chain(spheres.iter().map(|&pos| (pos, Rock::Sphere)))
            .collect::<BTreeMap<_, _>>();
        spheres = (0..max_row)
            .map(|row_idx| {
                let mut next_available_spot = 0;
                platform.iter().filter(|(pos, _)| pos.0 == row_idx).fold(
                    vec![],
                    |mut acc, ((_, col_idx), rock)| {
                        match rock {
                            Rock::Sphere => {
                                acc.push((row_idx, next_available_spot));
                                next_available_spot += 1;
                            }
                            Rock::Cube => {
                                next_available_spot = col_idx + 1;
                            }
                            Rock::Empty => {}
                        }
                        acc
                    },
                )
            })
            .flatten()
            .collect::<BTreeSet<_>>();

        platform = cube_rocks
            .iter()
            .map(|&pos| (pos, Rock::Cube))
            .chain(spheres.iter().map(|&pos| (pos, Rock::Sphere)))
            .collect::<BTreeMap<_, _>>();
        spheres = (0..max_col)
            .rev()
            .map(|col_idx| {
                let mut next_available_spot = max_row;
                platform
                    .iter()
                    .filter(|(pos, _)| pos.1 == col_idx)
                    .rev()
                    .fold(vec![], |mut acc, ((row_idx, _), rock)| {
                        match rock {
                            Rock::Sphere => {
                                acc.push((next_available_spot - 1, col_idx));
                                next_available_spot -= 1;
                            }
                            Rock::Cube => {
                                next_available_spot = *row_idx;
                            }
                            Rock::Empty => {}
                        }
                        acc
                    })
            })
            .flatten()
            .collect::<BTreeSet<_>>();

        platform = cube_rocks
            .iter()
            .map(|&pos| (pos, Rock::Cube))
            .chain(spheres.iter().map(|&pos| (pos, Rock::Sphere)))
            .collect::<BTreeMap<_, _>>();
        spheres = (0..max_row)
            .rev()
            .map(|row_idx| {
                let mut next_available_spot = max_col;
                platform
                    .iter()
                    .filter(|(pos, _)| pos.0 == row_idx)
                    .rev()
                    .fold(vec![], |mut acc, ((_, col_idx), rock)| {
                        match rock {
                            Rock::Sphere => {
                                acc.push((row_idx, next_available_spot - 1));
                                next_available_spot -= 1;
                            }
                            Rock::Cube => {
                                next_available_spot = *col_idx;
                            }
                            Rock::Empty => {}
                        }
                        acc
                    })
            })
            .flatten()
            .collect::<BTreeSet<_>>();

        start_bin_counts = Some((cycle, next_bin_counts.clone()));
        cycle_mem.insert(next_bin_counts, spheres.clone());
    }

    if let Some(cycle_length) = cycle_length {
        for _ in 0..((total_cycles - cycles_completed) % cycle_length) {
            let next_bin_counts = vertical_bins
                .iter()
                .map(|bin| {
                    spheres
                        .iter()
                        .filter(|(row_idx, col_idx)| {
                            *col_idx == bin.column && bin.row_range.contains(row_idx)
                        })
                        .count()
                })
                .collect_vec();
            let cycle_result = cycle_mem.get(&next_bin_counts).unwrap();

            spheres = cycle_result.clone();
        }
    }

    spheres
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(part2(input), 64);
    }
}
