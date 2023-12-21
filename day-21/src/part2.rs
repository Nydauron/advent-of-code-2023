use std::{
    collections::{HashMap, VecDeque},
    usize,
};

use itertools::Itertools;

pub fn part2(input: &str) -> usize {
    part2_steps(input, 26501365)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    GardenPlot,
    Rock,
    Start,
}

fn part2_steps(input: &str, steps: usize) -> usize {
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
    let max_rows = input.lines().count();
    let max_cols = input.lines().next().unwrap().len();
    assert_eq!(max_rows, max_cols);

    let origin_plot_distances = get_distances(&map, start_pos);

    let black_tiles = origin_plot_distances
        .iter()
        .filter(|(_, distance)| *distance % 2 == 0)
        .count();
    let red_tiles = origin_plot_distances
        .iter()
        .filter(|(_, distance)| *distance % 2 == 1)
        .count();

    let mut borders = HashMap::new();
    borders.extend((0..max_rows).map(|r| {
        let pos = (r, 0);
        (pos, origin_plot_distances.get(&pos).cloned().unwrap())
    }));
    borders.extend((0..max_rows).map(|r| {
        let pos = (r, max_cols - 1);
        (pos, origin_plot_distances.get(&pos).cloned().unwrap())
    }));
    borders.extend((1..max_cols - 1).map(|c| {
        let pos = (0, c);
        (pos, origin_plot_distances.get(&pos).cloned().unwrap())
    }));
    borders.extend((1..max_cols - 1).map(|c| {
        let pos = (max_rows - 1, c);
        (pos, origin_plot_distances.get(&pos).cloned().unwrap())
    }));

    let a = borders.iter().min_set_by(|a, b| a.1.cmp(b.1));

    // (border pos) -> HashMap<(usize, distance)>
    let mut border_distance_maps = HashMap::new();
    border_distance_maps.extend((0..max_rows).map(|r| {
        let pos = (r, 0);
        (pos, get_distances(&map, pos))
    }));
    border_distance_maps.extend((0..max_rows).map(|r| {
        let pos = (r, max_cols - 1);
        (pos, get_distances(&map, pos))
    }));
    border_distance_maps.extend((1..max_cols - 1).map(|c| {
        let pos = (0, c);
        (pos, get_distances(&map, pos))
    }));
    border_distance_maps.extend((1..max_cols - 1).map(|c| {
        let pos = (max_rows - 1, c);
        (pos, get_distances(&map, pos))
    }));

    let small_triangle = [
        (0, 0),
        (max_rows - 1, 0),
        (0, max_cols - 1),
        (max_rows - 1, max_cols - 1),
    ]
    .into_iter()
    .map(|start| {
        let valid_distances = border_distance_maps
            .get(&start)
            .unwrap()
            .iter()
            .filter(|(_, &distance)| distance < max_rows / 2)
            .collect_vec();
        (
            start,
            (
                valid_distances
                    .iter()
                    .filter(|(_, &distance)| distance % 2 == 0)
                    .count(),
                valid_distances
                    .iter()
                    .filter(|(_, &distance)| distance % 2 == 1)
                    .count(),
            ),
        )
    })
    .collect::<HashMap<_, _>>();

    let chopped_triangle = [
        (0, 0),
        (max_rows - 1, 0),
        (0, max_cols - 1),
        (max_rows - 1, max_cols - 1),
    ]
    .into_iter()
    .map(|start| {
        let valid_distances = border_distance_maps
            .get(&start)
            .unwrap()
            .iter()
            .filter(|(_, &distance)| distance < max_rows + max_rows / 2)
            .collect_vec();
        (
            start,
            (
                valid_distances
                    .iter()
                    .filter(|(_, &distance)| distance % 2 == 0)
                    .count(),
                valid_distances
                    .iter()
                    .filter(|(_, &distance)| distance % 2 == 1)
                    .count(),
            ),
        )
    })
    .collect::<HashMap<_, _>>();

    let side_plots = a
        .iter()
        .map(|(pos, _)| {
            let valid_distances = border_distance_maps
                .get(pos)
                .unwrap()
                .iter()
                .filter(|(_, &distance)| distance < max_rows)
                .collect_vec();
            (
                pos,
                (
                    valid_distances
                        .iter()
                        .filter(|(_, &distance)| distance % 2 == 0)
                        .count(),
                    valid_distances
                        .iter()
                        .filter(|(_, &distance)| distance % 2 == 1)
                        .count(),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    let plot_repetition: usize = (steps - max_rows / 2) / max_rows; // 202_300

    // Given the provided input.txt
    // each small triangle gets repeated 202_300 times (all opposite color)
    // each chopped triangle get repeated 202_300 - 1 times (all same color)
    // add the 4 directional plots (all same color)
    // 202_299 * (202_299 * 2) + (202_299 * 2 + 1) internal boards (checked)
    // (202_299 + 1) * (202_299 + 1) red boards (opposite boards)
    //
    // TOO HIGH: 594612339905084
    // TOO LOW:  594606461471777
    // TOO HIGH: 594606568488999
    // wrong:    594606568463233
    //           594606461446272
    //           594606455175141
    // CORRECT:  594606492802848
    //
    let red_internal_plots = plot_repetition.pow(2);
    let black_internal_plots = (plot_repetition - 1).pow(2);

    small_triangle
        .iter()
        .map(|(pos, (black, red))| {
            if origin_plot_distances.get(pos).unwrap() % 2 == steps % 2 {
                // println!(
                //     "small triangle counted as red {:?} x {}",
                //     pos, plot_repetition
                // );
                *red * plot_repetition
            } else {
                // println!(
                //     "small triangle counted as black {:?} x {}",
                //     pos, plot_repetition
                // );
                *black * plot_repetition
            }
        })
        .sum::<usize>()
        + chopped_triangle
            .iter()
            .map(|(pos, (black, red))| {
                if origin_plot_distances.get(pos).unwrap() % 2 == steps % 2 {
                    // println!(
                    //     "chopped triangle counted as black {:?} x {}",
                    //     pos, plot_repetition
                    // );
                    *black * (plot_repetition - 1)
                } else {
                    // println!(
                    //     "chopped triangle counted as red {:?} x {}",
                    //     pos, plot_repetition
                    // );
                    *red * (plot_repetition - 1)
                }
            })
            .sum::<usize>()
        + side_plots
            .iter()
            .map(|(pos, (black, red))| {
                if origin_plot_distances.get(pos).unwrap() % 2 == steps % 2 {
                    // println!("side plot counted as black {:?} x 1", pos,);
                    black
                } else {
                    // println!("side plot counted as red {:?} x 1", pos,);
                    red
                }
            })
            .sum::<usize>()
        + if steps % 2 == 0 {
            black_tiles
        } else {
            red_tiles
        } * black_internal_plots
        + if steps % 2 == 0 {
            red_tiles
        } else {
            black_tiles
        } * red_internal_plots
}

fn get_distances(map: &Vec<Vec<Tile>>, start: (usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut queue = VecDeque::from([(start, 0_usize)]);
    let mut visited_distance: HashMap<(usize, usize), usize> = HashMap::new();
    while let Some((curr_pos, curr_distance)) = queue.pop_front() {
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
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(6, 16)]
    #[case(10, 50)]
    #[case(50, 1594)]
    #[case(100, 6536)]
    #[case(500, 167004)]
    #[case(1000, 668697)]
    #[case(5000, 16733044)]
    fn test_part2(#[case] steps: usize, #[case] garden_plots_reached: usize) {
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

        assert_eq!(part2_steps(input, steps), garden_plots_reached);
    }
}
