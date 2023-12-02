use std::cmp::min;

fn main() {
    let input = include_str!("input.txt");
    let lines = input.split('\n');
    println!(
        "{}",
        lines
            .into_iter()
            .map(|a| -> u32 { part2(a) as u32 })
            .sum::<u32>()
    );
}

static VALID_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part2(line: &str) -> u16 {
    let mut first_num = 0;
    for (idx, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            first_num = c as u8 - b'0';
            break;
        }
        let slice_len = 5;
        let start_slice_idx = idx;
        let end_slice_idx = min(line.len() - 1, idx + slice_len);
        let slice = &line[start_slice_idx..end_slice_idx];
        let mut break_out = false;
        for (word_idx, word) in VALID_WORDS.into_iter().enumerate() {
            if slice.starts_with(word) {
                first_num = word_idx as u8 + 1;
                break_out = true;
                break;
            }
        }
        if break_out {
            break;
        }
    }
    let mut last_num = 0;
    for (idx, c) in line.chars().rev().enumerate() {
        if c.is_ascii_digit() {
            last_num = c as u8 - b'0';
            break;
        }
        let slice_len = 5;
        let start_slice_idx = line.len() - idx - 1;
        let end_slice_idx = min(line.len(), line.len() - idx - 1 + slice_len);
        let slice = &line[start_slice_idx..end_slice_idx];
        let mut break_out = false;
        for (word_idx, word) in VALID_WORDS.into_iter().enumerate() {
            if slice.starts_with(word) {
                last_num = word_idx as u8 + 1;
                break_out = true;
                break;
            }
        }
        if break_out {
            break;
        }
    }
    (first_num * 10) as u16 + last_num as u16
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("safg2fhkjw7fdhalkf", 27)]
    #[case("kjkjj5lkjf", 55)]
    #[case("ads2kljh4jkldfas9ashdklj5dasdlk", 25)]
    #[case("hklekogfajlekopgscgkl", 0)]
    fn test_part1_still_works(#[case] line: &str, #[case] expected: u16) {
        assert_eq!(part2(line), expected);
    }

    #[rstest]
    #[case("hkthreeogfajlekopfourgkl", 34)]
    #[case("sdgsljksixlfdjk", 66)]
    #[case("dsgf4dhglkftwothree9asdkjlheightdsad", 48)]
    #[case("4dhglktwothree9asdkjlheightdsad2", 42)]
    #[case("ninedhglktwothree9asdkjlheightdsadthree", 93)]
    fn test_part2(#[case] line: &str, #[case] expected: u16) {
        assert_eq!(part2(line), expected);
    }
}
