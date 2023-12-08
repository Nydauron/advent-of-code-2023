use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag, character::complete::alphanumeric1, sequence::separated_pair, IResult,
};

pub fn part2(input: &str) -> u64 {
    let map = Map::parse_map(input);

    let mut steps = 0;
    let mut current_nodes = map.start_nodes.into_iter().collect::<HashSet<_>>();
    loop {
        for direction in &map.pattern {
            if current_nodes
                .iter()
                .map(|&node| node.chars().last().expect("node name is empty") == 'Z')
                .all(|char_equals_z| char_equals_z)
            {
                return steps;
            }
            steps += 1;
            if steps % 10000000_u64 == 0 {
                println!("{:?}", &current_nodes);
                println!("Step count: {}", steps);
            }
            current_nodes = current_nodes
                .iter()
                .map(|node| {
                    let node = map.nodes.get(node).expect("Could not find node");
                    match direction {
                        Direction::Left => node.left,
                        Direction::Right => node.right,
                    }
                })
                .collect::<HashSet<_>>();
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
                (node.chars().last().expect("Node during parsing is empty") == 'A').then_some(node)
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
