use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    let lines = input.lines();

    lines.map(part1_line).sum()
}

fn part1_line(line: &str) -> u32 {
    let (_, card_numbers) = line.split_once(": ").expect("not valid game");
    let (winning_numbers_str, my_numbers_str) = card_numbers
        .split_once(" | ")
        .expect("No | delimiter found");
    let winning_numbers = winning_numbers_str
        .split_whitespace()
        .map(|a| a.parse::<u32>().expect("Parsing winning number failed"))
        .collect::<HashSet<_>>();
    let my_numbers = my_numbers_str
        .split_whitespace()
        .map(|a| a.parse::<u32>().expect("Parsing winning number failed"))
        .collect::<HashSet<_>>();

    let count = winning_numbers.intersection(&my_numbers).count() as u32;
    if count > 0 {
        (2_u32).pow(count - 1)
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), 13);
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    fn test_part1_line(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part1_line(input), expected);
    }
}
