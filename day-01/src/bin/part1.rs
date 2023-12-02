fn main() {
    let input = include_str!("input.txt");
    let lines = input.split('\n');
    println!(
        "{}",
        lines.into_iter().map(|a| -> u16 { part1(a) }).sum::<u16>()
    );
}

fn part1(line: &str) -> u16 {
    let mut first_num = 0;
    for c in line.chars() {
        if c.is_ascii_digit() {
            first_num = c as u8 - b'0';
            break;
        }
    }
    let mut last_num = 0;
    for c in line.chars().rev() {
        if c.is_ascii_digit() {
            last_num = c as u8 - b'0';
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
    fn test_part1(#[case] line: &str, #[case] expected: u16) {
        assert_eq!(part1(line), expected);
    }
}
