use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1},
    sequence::preceded,
    IResult, Parser,
};

pub fn part2(input: &str) -> usize {
    const EMPTY_BOX: Vec<(&str, u32)> = Vec::new();
    let mut boxes = vec![EMPTY_BOX; 256];
    let sequences = input
        .split(',')
        .map(|seq| {
            let (_, seq) = parse_seqence(seq).expect("seqence parsed incorrectly");
            seq
        })
        .collect_vec();

    sequences.iter().for_each(|sequence| {
        let selected_box = &mut boxes[sequence.hash as usize];
        match sequence.operation {
            Operation::Set(value) => {
                let existing_lens = selected_box
                    .iter_mut()
                    .find(|lens| lens.0 == sequence.ident);
                if let Some(lens) = existing_lens {
                    lens.1 = value;
                } else {
                    selected_box.push((sequence.ident, value));
                }
            }
            Operation::Remove => {
                if let Some(lens_idx) = selected_box
                    .iter()
                    .position(|lens| lens.0 == sequence.ident)
                {
                    selected_box.remove(lens_idx);
                }
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_idx, b)| {
            b.iter()
                .enumerate()
                .map(move |(lens_idx, lens)| (box_idx + 1) * (lens_idx + 1) * lens.1 as usize)
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Remove,
    Set(u32),
}

#[derive(Debug, Clone, Copy)]
struct Sequence<'a> {
    ident: &'a str,
    hash: u8,
    operation: Operation,
}

fn parse_seqence<'a>(input: &'a str) -> IResult<&'a str, Sequence<'a>> {
    let (input, (ident, hash)) =
        alpha1(input).map(|(input, ident)| (input, (ident, calculate_hash(ident))))?;
    let (input, operation) = tag("-")
        .map(|_| Operation::Remove)
        .or(preceded(tag("="), complete::u32).map(|value| Operation::Set(value)))
        .parse(input)?;

    IResult::Ok((
        input,
        Sequence {
            ident,
            hash,
            operation,
        },
    ))
}

pub fn calculate_hash(input: &str) -> u8 {
    input.chars().fold(0_u8, |acc, c| {
        if c == '\n' {
            acc
        } else {
            if let Some(ascii) = c.as_ascii() {
                acc.wrapping_add(ascii as u8).wrapping_mul(17)
            } else {
                acc
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(part2(input), 145);
    }

    #[test]
    fn test_hash_calculation() {
        let input = "HASH";

        assert_eq!(calculate_hash(input), 52);
    }
}
