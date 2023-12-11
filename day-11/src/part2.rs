use itertools::Itertools;
use std::collections::BTreeMap;

pub fn part2(input: &str) -> i64 {
    part2_helper(input, 1_000_000)
}

pub fn part2_helper(input: &str, scale_factor: usize) -> i64 {
    let mut space = input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col_idx, c)| match c {
                    '#' => Some(((row_idx, col_idx), SpaceTile::Galaxy)),
                    '.' => None,
                    c => panic!("Not a valid character: {}", c),
                })
        })
        .collect::<BTreeMap<_, _>>();

    let max_row = input.lines().count();
    let max_col = input.lines().map(|line| line.len()).max().unwrap();
    let mut no_galaxy_rows = (0..max_row)
        .filter(|r| !(0..max_col).any(|c| space.contains_key(&(*r, c))))
        .collect::<Vec<_>>();
    let mut no_galaxy_cols = (0..max_col)
        .filter(|c| !(0..max_row).any(|r| space.contains_key(&(r, *c))))
        .collect::<Vec<_>>();

    let mut new_max_row = max_row;
    for r in (0..max_row).rev() {
        if no_galaxy_rows.is_empty() {
            break;
        }
        while let Some(no_galaxy_row) = no_galaxy_rows.last() {
            if r >= *no_galaxy_row {
                break;
            }
            no_galaxy_rows.pop();
        }
        for c in 0..max_col {
            if let Some((key, space_tile)) = space.remove_entry(&(r, c)) {
                let new_key = (key.0 + (no_galaxy_rows.len() * (scale_factor - 1)), key.1);
                new_max_row = new_max_row.max(new_key.0);
                space.insert(new_key, space_tile);
            }
        }
    }

    for c in (0..max_col).rev() {
        if no_galaxy_cols.is_empty() {
            break;
        }
        while let Some(no_galaxy_col) = no_galaxy_cols.last() {
            if c >= *no_galaxy_col {
                break;
            }
            no_galaxy_cols.pop();
        }
        let galaxies = space
            .keys()
            .filter(|pos| pos.1 == c)
            .cloned()
            .collect::<Vec<_>>();
        galaxies
            .into_iter()
            .filter_map(|key| space.remove_entry(&key))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(key, space_tile)| {
                let new_key = (key.0, key.1 + (no_galaxy_cols.len() * (scale_factor - 1)));
                space.insert(new_key, space_tile);
            });
    }

    space
        .keys()
        .combinations(2)
        .map(|galaxies| {
            debug_assert_eq!(galaxies.len(), 2);
            let a = galaxies[0];
            let b = galaxies[1];

            (b.0 as i64 - a.0 as i64).abs() + (b.1 as i64 - a.1 as i64).abs()
        })
        .sum::<i64>()
}

#[derive(Debug, Copy, Clone)]
enum SpaceTile {
    Galaxy,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2_scale_100() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(part2_helper(input, 100), 8410);
    }
    #[test]
    fn test_part2_scale_10() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(part2_helper(input, 10), 1030);
    }
}
