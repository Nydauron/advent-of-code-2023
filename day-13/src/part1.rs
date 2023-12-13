use std::collections::HashSet;

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input.split("\n\n").map(part1_pattern).sum()
}

fn part1_pattern(pattern: &str) -> usize {
    let pattern = pattern
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let canidate_vertical_mirrors = pattern
        .iter()
        .map(|row| {
            (1..row.len())
                .filter(|canidate_vertical_mirror| {
                    for offset in 0..row.len() {
                        let left_chat_idx = canidate_vertical_mirror - offset - 1;
                        let right_char_idx = canidate_vertical_mirror + offset;
                        if right_char_idx >= row.len() {
                            break;
                        }

                        if row[right_char_idx] != row[left_chat_idx] {
                            return false;
                        }

                        if left_chat_idx == 0 {
                            break;
                        }
                    }
                    true
                })
                .collect::<HashSet<_>>()
        })
        .reduce(|acc, canidate_mirrors| acc.intersection(&canidate_mirrors).cloned().collect())
        .unwrap()
        .into_iter()
        .collect_vec();
    if let Some(&mirror_idx) = canidate_vertical_mirrors.first() {
        return mirror_idx;
    }
    let canidate_horizontal_mirrors = (0..pattern[0].len())
        .map(|c| {
            (1..pattern.len())
                .filter(|canidate_horizontal_mirror| {
                    for offset in 0..pattern.len() {
                        let left_chat_idx = canidate_horizontal_mirror - offset - 1;
                        let right_char_idx = canidate_horizontal_mirror + offset;
                        if right_char_idx >= pattern.len() {
                            break;
                        }

                        if pattern[right_char_idx][c] != pattern[left_chat_idx][c] {
                            return false;
                        }

                        if left_chat_idx == 0 {
                            break;
                        }
                    }
                    true
                })
                .collect::<HashSet<_>>()
        })
        .reduce(|acc, canidate_mirrors| acc.intersection(&canidate_mirrors).cloned().collect())
        .unwrap()
        .into_iter()
        .collect_vec();
    if let Some(&mirror_idx) = canidate_horizontal_mirrors.first() {
        return mirror_idx * 100;
    }
    todo!()
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        5
    )]
    #[case(
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        400 // 4 * 100
    )]
    #[case(
        "..###..###...##
...#....#....##
.#........#.###
..#.#..#.#..###
#...#..#...##..
###.#..#.###.##
##############.
#.#.#..#.#.#.##
..##.##.##.....
",
        6
    )]
    #[case(
        "#.######.##
...####....
...#..#....
.#..##..#..
##.#..#.###
#.#....#.##
#.#......##
####..#####
#.#....#.##
.#.####.#..
#.##..##.##
##..##..###
.##.##.##..",
        10
    )]
    fn test_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part1_pattern(input), expected);
    }
}
