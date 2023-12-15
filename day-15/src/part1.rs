pub fn part1(input: &str) -> u32 {
    input.split(',').map(|seq| calculate_hash(seq)).sum()
}

fn calculate_hash(input: &str) -> u32 {
    input.chars().fold(0_u32, |mut acc, c| {
        if c == '\n' {
            acc
        } else {
            if let Some(ascii) = c.as_ascii() {
                let value = ascii as u8;
                acc += value as u32;
                acc *= 17;
                acc %= 256;
            }
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn test_hash_calculation() {
        let input = "HASH";

        assert_eq!(calculate_hash(input), 52);
    }
}
