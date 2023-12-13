use std::collections::HashSet;

use itertools::Itertools;

pub fn part2(input: &str) -> usize {
    input.split("\n\n").map(part2_pattern).sum()
}

fn part2_pattern(pattern: &str) -> usize {
    let pattern = pattern
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let original_mirror = find_mirror(&pattern).unwrap();
    let mirrors = pattern
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            let pattern = &pattern;
            row.iter().enumerate().filter_map(move |(col_idx, _)| {
                let canidate_pos = (row_idx, col_idx);
                find_mirror_with_smudge(pattern, canidate_pos, original_mirror)
            })
        })
        .collect::<HashSet<_>>();

    mirrors
        .into_iter()
        .exactly_one()
        .expect(format!("{:?}", pattern).as_str())
        .clone()
}

fn find_mirror(pattern: &Vec<Vec<char>>) -> Option<usize> {
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
        return Some(mirror_idx);
    }
    let canidate_horizontal_mirrors = (0..pattern[0].len())
        .map(|col_idx| {
            (1..pattern.len())
                .filter(|canidate_horizontal_mirror| {
                    for offset in 0..pattern.len() {
                        let left_chat_idx = canidate_horizontal_mirror - offset - 1;
                        let right_char_idx = canidate_horizontal_mirror + offset;
                        if right_char_idx >= pattern.len() {
                            break;
                        }

                        if pattern[right_char_idx][col_idx] != pattern[left_chat_idx][col_idx] {
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
        return Some(mirror_idx * 100);
    }
    None
}
fn find_mirror_with_smudge(
    pattern: &Vec<Vec<char>>,
    canidate_smduge: (usize, usize),
    mirror_value: usize,
) -> Option<usize> {
    let canidate_vertical_mirrors = pattern
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            (1..row.len())
                .filter(|canidate_vertical_mirror| {
                    for offset in 0..row.len() {
                        let left_chat_idx = canidate_vertical_mirror - offset - 1;
                        let right_char_idx = canidate_vertical_mirror + offset;
                        if right_char_idx >= row.len() {
                            break;
                        }

                        let is_one_smudged = row_idx == canidate_smduge.0
                            && (left_chat_idx == canidate_smduge.1
                                || right_char_idx == canidate_smduge.1);

                        if is_one_smudged && row[right_char_idx] == row[left_chat_idx]
                            || !is_one_smudged && row[right_char_idx] != row[left_chat_idx]
                        {
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
        .filter(|&mirror_idx| mirror_idx != (mirror_value % 100))
        .collect_vec();
    if let Some(&mirror_idx) = canidate_vertical_mirrors.first() {
        return Some(mirror_idx);
    }
    let canidate_horizontal_mirrors = (0..pattern[0].len())
        .map(|col_idx| {
            (1..pattern.len())
                .filter(|canidate_horizontal_mirror| {
                    for offset in 0..pattern.len() {
                        let left_chat_idx = canidate_horizontal_mirror - offset - 1;
                        let right_char_idx = canidate_horizontal_mirror + offset;
                        if right_char_idx >= pattern.len() {
                            break;
                        }

                        let is_one_smudged = col_idx == canidate_smduge.1
                            && (left_chat_idx == canidate_smduge.0
                                || right_char_idx == canidate_smduge.0);

                        if is_one_smudged
                            && pattern[right_char_idx][col_idx] == pattern[left_chat_idx][col_idx]
                            || !is_one_smudged
                                && pattern[right_char_idx][col_idx]
                                    != pattern[left_chat_idx][col_idx]
                        {
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
        .filter(|&mirror_idx| mirror_idx != (mirror_value / 100))
        .collect_vec();
    if let Some(&mirror_idx) = canidate_horizontal_mirrors.first() {
        return Some(mirror_idx * 100);
    }
    None
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
        300 // 3 * 100
    )]
    #[case(
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        100 // 1 * 100
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
..##.##.##.....",
        14
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
        5
    )]
    fn test_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part2_pattern(input), expected);
    }
}
