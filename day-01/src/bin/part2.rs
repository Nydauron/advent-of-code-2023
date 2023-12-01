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

    #[test]
    fn test_2_number_string() {
        let input = "safg2fhkjw7fdhalkf";
        assert_eq!(part2(input), 27);
    }
    #[test]
    fn test_1_number_string() {
        let input = "kjkjj5lkjf";
        assert_eq!(part2(input), 55);
    }
    #[test]
    fn test_many_number_string() {
        let input = "ads2kljh4jkldfas9ashdklj5dasdlk";
        assert_eq!(part2(input), 25);
    }
    #[test]
    fn test_no_number_string() {
        let input = "hklekogfajlekopgscgkl";
        assert_eq!(part2(input), 0);
    }
    #[test]
    fn test_2_word_number_string() {
        let input = "hkthreeogfajlekopfourgkl";
        assert_eq!(part2(input), 34);
    }
    #[test]
    fn test_1_word_number_string() {
        let input = "sdgsljksixlfdjk";
        assert_eq!(part2(input), 66);
    }
    #[test]
    fn test_many_words_and_numbers_string() {
        let input = "dsgf4dhglkftwothree9asdkjlheightdsad";
        assert_eq!(part2(input), 48);
    }
    #[test]
    fn test_many_words_with_number_ends_string() {
        let input = "4dhglktwothree9asdkjlheightdsad2";
        assert_eq!(part2(input), 42);
    }
    #[test]
    fn test_with_word_ends_string() {
        let input = "ninedhglktwothree9asdkjlheightdsadthree";
        assert_eq!(part2(input), 93);
    }
}
