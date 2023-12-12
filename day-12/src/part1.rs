use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn part1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| line.split_once(' ').expect("No space delimiter"))
        .collect_vec();

    lines
        .par_iter()
        .map(|(record, group_str)| {
            let groups = group_str
                .split(',')
                .map(|g| g.parse::<u32>().expect("Grouping is not a number"))
                .collect_vec();

            evaluate_record_pattern(record, &groups)
        })
        .sum()
}

fn evaluate_record_pattern(partial_record: &str, groupings: &Vec<u32>) -> usize {
    let question_mark_idxes = partial_record
        .char_indices()
        .filter_map(|(idx, c)| (c == '?').then_some(idx))
        .enumerate()
        .map(|(idx, record_idx)| (record_idx, idx))
        .collect::<HashMap<_, _>>();
    let count = (0..2_u64.pow(question_mark_idxes.len() as u32))
        .filter(|permutation| {
            // create strings by substituting ?
            let constructed_string = partial_record
                .char_indices()
                .map(|(idx, c)| {
                    if let Some(bit_offset) = question_mark_idxes.get(&idx) {
                        let mask = 1 << bit_offset;
                        if permutation & mask != 0 {
                            return '#';
                        }
                    }
                    c
                })
                .collect::<String>()
                .replace("?", ".");

            // store in iterator and map over all checking if true
            // return count number of successful checks
            let is_valid = check_damage_record(constructed_string.as_str(), groupings);
            // println!("{}: {}", constructed_string, is_valid);
            is_valid
        })
        .count();
    dbg!(count)
}

fn check_damage_record(record: &str, groupings: &Vec<u32>) -> bool {
    let damage_idxes = record
        .char_indices()
        .filter_map(|(idx, c)| (c == '#').then_some(idx))
        .collect_vec();
    let damage_ids = damage_idxes
        .iter()
        .enumerate()
        .map(|(idx, damage_idx)| damage_idx - idx)
        .fold(BTreeMap::<usize, u32>::new(), |mut acc, damage_id| {
            acc.entry(damage_id)
                .and_modify(|curr| *curr += 1)
                .or_insert(1);

            acc
        });

    let is_valid = damage_ids
        .values()
        .zip(groupings.iter())
        .all(|(id_count, group_count)| id_count == group_count)
        && damage_ids.len() == groupings.len();

    is_valid
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(part1(input), 21);
    }
}
