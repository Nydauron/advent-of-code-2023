use std::collections::HashMap;

use nom::{bytes::complete::tag, character::complete::alpha1, sequence::separated_pair, IResult};

pub fn part1(input: &str) -> u32 {
    let map = Map::parse_map(input);

    let mut steps = 0;
    let mut current_node = "AAA";
    loop {
        for direction in &map.pattern {
            let node = map.nodes.get(current_node).expect("Could not find node");
            if node.name == "ZZZ" {
                return steps;
            }
            steps += 1;
            match direction {
                Direction::Left => {
                    current_node = node.left;
                }
                Direction::Right => {
                    current_node = node.right;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Map<'a> {
    pattern: Vec<Direction>,
    nodes: HashMap<&'a str, Node<'a>>,
}

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse_node<'a>(line: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    let (line, _) = tag("(")(line)?;
    let (input, (left, right)) = separated_pair(alpha1, tag(", "), alpha1)(line)?;
    Ok((input, (left, right)))
}
impl<'a> Map<'a> {
    fn parse_map(input: &'a str) -> Self {
        let (pattern, nodes_str) = input.split_once("\n\n").expect("Could not parse correctly");
        let pattern = pattern
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                c => panic!("Unknown character found: {}", c),
            })
            .collect::<Vec<_>>();

        let nodes = nodes_str
            .lines()
            .map(|line: &'a str| {
                let (node_name, edges) = line.split_once("=").expect("Could not split node line");
                let (_, (left, right)) = parse_node(edges.trim()).expect("Could not parse edges");

                let node_name = node_name.trim();
                (
                    node_name,
                    Node {
                        name: node_name,
                        left,
                        right,
                    },
                )
            })
            .collect::<HashMap<&'a str, Node<'a>>>();

        Self { pattern, nodes }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1_part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 2);
    }
    #[test]
    fn test2_part1() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 6);
    }
}
