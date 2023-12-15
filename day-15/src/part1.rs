pub fn part1(input: &str) -> u32 {
    sum(input
        .chars()
        .fold((0, 0_u8), |(total, curr_hash_total), c| match c {
            ',' => (total + curr_hash_total as u32, 0),
            '\n' => (total, curr_hash_total),
            c => {
                if let Some(ascii) = c.as_ascii() {
                    let value = ascii as u8;
                    (total, curr_hash_total.wrapping_add(value).wrapping_mul(17))
                } else {
                    (total, curr_hash_total)
                }
            }
        }))
}

#[inline]
fn sum((a, b): (u32, u8)) -> u32 {
    a + b as u32
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
