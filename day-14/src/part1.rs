use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let platform = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    (0..platform[0].len())
        .map(|col_idx| {
            let mut next_available_spot = 0;
            let mut total_weight = 0;
            for (row_idx, row) in platform.iter().enumerate() {
                let rock = row[col_idx];
                match rock {
                    'O' => {
                        total_weight += platform.len() - next_available_spot;
                        next_available_spot += 1;
                    }
                    '#' => {
                        next_available_spot = row_idx + 1;
                    }
                    '.' => {}
                    c => panic!("Unknown char found: {}", c),
                }
            }
            total_weight
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
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

        assert_eq!(part1(input), 136);
    }
}
