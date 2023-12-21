use std::collections::{HashMap, VecDeque};

pub fn part1(input: &str) -> usize {
    part1_steps(input, 64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    GardenPlot,
    Rock,
    Start,
}
fn part1_steps(input: &str, steps: usize) -> usize {
    let (map, start_pos) = input
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| match c {
                    '.' => (Tile::GardenPlot, None),
                    '#' => (Tile::Rock, None),
                    'S' => (Tile::Start, Some((row_idx, col_idx))),
                    c => panic!("Encountered unexpected char: {}", c),
                })
                .fold(
                    (vec![], None),
                    |(mut acc, start), (tile, possible_start)| {
                        acc.push(tile);
                        match start {
                            None => (acc, possible_start),
                            Some(pos) => (acc, Some(pos)),
                        }
                    },
                )
        })
        .fold((vec![], None), |(mut acc, start), (row, possible_start)| {
            acc.push(row);
            match start {
                None => (acc, possible_start),
                Some(pos) => (acc, Some(pos)),
            }
        });
    let start_pos = start_pos.unwrap();
    // let max_rows = input.lines().count();
    // let max_cols = input.lines().next().unwrap().len();

    let mut queue = VecDeque::from([(start_pos, 0_usize)]);
    let mut visited_distance: HashMap<(usize, usize), usize> = HashMap::new();
    while let Some((curr_pos, curr_distance)) = queue.pop_front() {
        if curr_distance > steps {
            break;
        }
        if visited_distance.contains_key(&curr_pos) {
            continue;
        }
        visited_distance.insert(curr_pos, curr_distance);
        let offset_by = 1;
        let curr_possible_directions = [
            (offset_by <= curr_pos.0).then(|| (curr_pos.0 - offset_by, curr_pos.1)),
            Some((curr_pos.0 + offset_by, curr_pos.1)),
            Some((curr_pos.0, curr_pos.1 + offset_by)),
            (offset_by <= curr_pos.1).then(|| (curr_pos.0, curr_pos.1 - offset_by)),
        ];
        for next_pos in curr_possible_directions {
            if let Some(next_pos) = next_pos {
                if let Some(row) = map.get(next_pos.0) {
                    if let Some(tile) = row.get(next_pos.1) {
                        match tile {
                            Tile::Rock => {}
                            Tile::GardenPlot | Tile::Start => {
                                queue.push_back(((next_pos), curr_distance + 1));
                            }
                        }
                    }
                }
            }
        }
    }

    visited_distance
        .iter()
        .filter(|(_, distance)| *distance % 2 == steps % 2)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        assert_eq!(part1_steps(input, 6), 16);
    }
}
