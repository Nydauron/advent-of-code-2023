use std::collections::BTreeMap;

use itertools::Itertools;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn part2(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| line.split_once(' ').expect("No space delimiter"))
        .collect_vec();

    lines
        .par_iter()
        .map(|(record, group_str)| {
            let record = [record, record, record, record, record].iter().join("?");
            let groups = [group_str, group_str, group_str, group_str, group_str]
                .iter()
                .join(",")
                .split(',')
                .map(|g| g.parse::<usize>().expect("Grouping is not a number"))
                .collect_vec();

            evaluate_record_pattern(record.as_str(), groups.as_slice())
        })
        .sum()
}

fn evaluate_record_pattern(partial_record: &str, groupings: &[usize]) -> usize {
    let mut mem = BTreeMap::new();

    for (record_start, record_slice) in (0..=partial_record.len())
        .map(|start| (start, &partial_record[start..]))
        .rev()
    {
        if record_slice.contains('#') {
            continue;
        }
        mem.insert((record_start, groupings.len()), 1);
    }

    for (grouping_start, grouping_slice) in (0..groupings.len())
        .map(|start| (start, &groupings[start..]))
        .rev()
    {
        let group_size = grouping_slice.first().unwrap();
        for (record_start, record_slice, suffix_char) in (0..partial_record.len())
            .filter_map(|start| {
                (start + *group_size <= partial_record.len()).then(|| {
                    (
                        start,
                        &partial_record[start..(start + *group_size)],
                        if start + group_size > 0 {
                            partial_record.chars().nth(start + group_size)
                        } else {
                            None
                        },
                    )
                })
            })
            .rev()
        {
            let mut count = if record_slice.chars().nth(0).unwrap() != '#' {
                mem.get(&(partial_record.len().min(record_start + 1), grouping_start))
                    .cloned()
                    .unwrap_or(0)
            } else {
                0
            };
            let window_end = *grouping_slice.first().unwrap();
            if !record_slice.contains('.')
                && suffix_char.and_then(|c| Some(c != '#')).unwrap_or(true)
            {
                count += mem
                    .get(&(
                        partial_record.len().min(record_start + window_end + 1),
                        grouping_start + 1,
                    ))
                    .unwrap_or(&0);
            }

            if count > 0 {
                mem.insert((record_start, grouping_start), count);
            }
        }
    }
    mem.get(&(0, 0)).cloned().unwrap()
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[test]
    #[ignore]
    fn test_part2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(part2(input), 525152);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn test_individual_lines(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part2(input), expected)
    }
}
