fn main() {
    let input = include_str!("input.txt");
    let lines = input.split("\n");
    println!("{}", lines.into_iter().map(|a| -> u16 {part1(a)}).fold(0, |acc, num| -> u16 {acc + num}))
}


fn part1(line: &str) -> u16 {
    let mut first_num = 0;
    for c in line.chars() {
        if c >= '0' && c <='9' {
            first_num = c as u8 - '0' as u8;
            break;
        }
    }
    let mut last_num = 0;
    for c in line.chars().rev() {
        if c >= '0' && c <='9' {
            last_num = c as u8 - '0' as u8;
            break;
        }
    }
    return (first_num * 10) as u16 + last_num as u16;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2_number_string() {
        let input = "safg2fhkjw7fdhalkf";
        assert_eq!(part1(input), 27);
    }
    #[test]
    fn test_1_number_string() {
        let input = "kjkjj5lkjf";
        assert_eq!(part1(input), 55);
    }
    #[test]
    fn test_many_number_string() {
        let input = "ads2kljh4jkldfas9ashdklj5dasdlk";
        assert_eq!(part1(input), 25);
    }
    #[test]
    fn test_no_number_string() {
        let input = "hklekogfajlekopgscgkl";
        assert_eq!(part1(input), 0);
    }
}