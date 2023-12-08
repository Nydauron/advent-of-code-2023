use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::alphanumeric1, sequence::separated_pair, IResult,
};
use num::integer::lcm;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// Unfortunately this problem wasn't worded to assume that the path offsets from the cycle that
/// contains XXZ was equal. Upon further inspection of the input, the input just so happened to be
/// six cycles each containing one XXA and XXZ.
///
/// The reason why LCM holds for this case is due to there being no offset. If there was an offset,
/// then the current only other way to do it is by brute forcing iterating through the graph.
pub fn part2(input: &str) -> u64 {
    let map = Map::parse_map(input);

    let cycles = map
        .start_nodes
        .par_iter()
        .map(|&node| {
            let mut curr_node = node;
            let a = map
                .pattern
                .iter()
                .cycle()
                .position(|direction| {
                    let lookedup_node = map
                        .nodes
                        .get(curr_node)
                        .expect("node not found in node map");
                    curr_node = match direction {
                        Direction::Left => lookedup_node.left,
                        Direction::Right => lookedup_node.right,
                    };
                    curr_node.ends_with('Z')
                })
                .unwrap()
                + 1;
            a
        })
        .reduce(|| 1, lcm);
    return cycles as u64;
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Map<'a> {
    pattern: Vec<Direction>,
    start_nodes: Vec<&'a str>,
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
    let (input, (left, right)) = separated_pair(alphanumeric1, tag(", "), alphanumeric1)(line)?;
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

        let start_nodes = nodes
            .keys()
            .filter_map(|&node| {
                (node.chars().last().expect("node during parsing is empty") == 'A').then_some(node)
            })
            .collect::<Vec<_>>();
        Self {
            pattern,
            start_nodes,
            nodes,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(part2(input), 6);
    }

    #[test]
    fn test_offset_part2() {
        let input = "LR

11A = (11D, XXX)
11D = (XXX, 11C)
11C = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22E, XXX)
22E = (XXX, 22D)
22D = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(part2(input), 8);
    }
}
