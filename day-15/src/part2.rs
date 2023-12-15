use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1},
    sequence::preceded,
    IResult, Parser,
};

pub fn part2(input: &str) -> usize {
    let mut boxes: [Vec<(&str, u32)>; 256] = (0..256)
        .map(|_| Vec::new())
        .collect_vec()
        .try_into()
        .unwrap();
    let sequences = input
        .split(',')
        .map(|seq| {
            let (_, seq) = parse_seqence(seq).expect("seqence parsed incorrectly");
            seq
        })
        .collect_vec();

    sequences.iter().for_each(|sequence| {
        let hash = calculate_hash(sequence.ident);
        let selected_box = &mut boxes[hash as usize];
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
    operation: Operation,
}

fn parse_seqence<'a>(input: &'a str) -> IResult<&'a str, Sequence<'a>> {
    let (input, ident) = alpha1(input)?;
    let (input, operation) = tag("-")
        .map(|_| Operation::Remove)
        .or(preceded(tag("="), complete::u32).map(|value| Operation::Set(value)))
        .parse(input)?;

    IResult::Ok((input, Sequence { ident, operation }))
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
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(part2(input), 145);
    }
}
