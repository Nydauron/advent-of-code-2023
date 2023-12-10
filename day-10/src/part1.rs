use std::collections::{HashMap, VecDeque};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn part1(input: &str) -> usize {
    let (pipes, start_pos) = input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line.chars().enumerate().map(move |(col_idx, c)| {
                use Pipe::*;
                let pos = (row_idx, col_idx);
                match c {
                    '|' => ((pos, Vertical), None),
                    '-' => ((pos, Horizontal), None),
                    'L' => ((pos, L), None),
                    'J' => ((pos, J), None),
                    '7' => ((pos, Seven), None),
                    'F' => ((pos, F), None),
                    '.' => ((pos, Ground), None),
                    'S' => ((pos, Start), Some((row_idx, col_idx))),
                    c => panic!("Unexpected char found: {}", c),
                }
            })
        })
        .fold((HashMap::new(), None), |mut acc, (pipe, start_pos)| {
            acc.0.insert(pipe.0, pipe.1);
            if acc.1.is_none() {
                acc.1 = start_pos;
            }
            acc
        });

    let start_pos = start_pos.expect("No start point found");

    [
        PipeNode {
            pos: start_pos.clone(),
            going: Direction::Up,
        },
        PipeNode {
            pos: start_pos.clone(),
            going: Direction::Left,
        },
        PipeNode {
            pos: start_pos.clone(),
            going: Direction::Down,
        },
        PipeNode {
            pos: start_pos.clone(),
            going: Direction::Right,
        },
    ]
    .into_par_iter()
    .filter_map(|node| check_next(&node, &pipes))
    .filter_map(|next_start| {
        let mut stack = VecDeque::from([next_start.clone()]);

        let mut found_path = false;
        while !stack.is_empty() {
            if let Some(curr) = stack.pop_back() {
                stack.push_back(curr);
                if let Some(pipe) = pipes.get(&curr.pos) {
                    use Pipe::*;
                    if match pipe {
                        Vertical | Horizontal | L | J | Seven | F => {
                            if let Some(next_node) = check_next(&curr, &pipes) {
                                stack.push_back(next_node);
                            } else {
                                return None;
                            }
                            false
                        }
                        Start => true,
                        Ground => false,
                    } {
                        found_path = true;
                        break;
                    }
                }
            }
        }
        if found_path {
            Some(stack.len())
        } else {
            None
        }
    })
    .max()
    .unwrap()
        / 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct PipeNode {
    pos: (usize, usize),
    going: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    L,
    J,
    Seven,
    F,
    Ground,
    Start,
}

fn check_next(node: &PipeNode, pipes: &HashMap<(usize, usize), Pipe>) -> Option<PipeNode> {
    use Pipe::*;
    let next_position = match node.going {
        Direction::Up => (
            if node.pos.0 > 0 {
                node.pos.0 - 1
            } else {
                return None;
            },
            node.pos.1,
        ),
        Direction::Down => (node.pos.0 + 1, node.pos.1),
        Direction::Left => (
            node.pos.0,
            if node.pos.1 > 0 {
                node.pos.1 - 1
            } else {
                return None;
            },
        ),
        Direction::Right => (node.pos.0, node.pos.1 + 1),
    };
    let acceptable_pipe_types = match node.going {
        Direction::Up => vec![Vertical, Seven, F, Start],
        Direction::Down => vec![Vertical, L, J, Start],
        Direction::Left => vec![Horizontal, L, F, Start],
        Direction::Right => vec![Horizontal, Seven, J, Start],
    };

    pipes.get(&next_position).and_then(|pipe_type| {
        if acceptable_pipe_types.contains(pipe_type) {
            let going_next = get_next_going_direction(pipe_type, node.going);
            Some(PipeNode {
                pos: next_position,
                going: going_next,
            })
        } else {
            None
        }
    })
}

fn get_next_going_direction(pipe: &Pipe, incoming_going: Direction) -> Direction {
    use Pipe::*;
    match pipe {
        Vertical => match incoming_going {
            Direction::Up => Direction::Up,
            Direction::Down => Direction::Down,
            _ => panic!("not valid incoming direction"),
        },
        Horizontal => match incoming_going {
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
            _ => panic!("not valid incoming direction"),
        },
        L => match incoming_going {
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            _ => panic!("not valid incoming direction"),
        },
        J => match incoming_going {
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Up,
            _ => panic!("not valid incoming direction"),
        },
        Seven => match incoming_going {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Down,
            _ => panic!("not valid incoming direction"),
        },
        F => match incoming_going {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Down,
            _ => panic!("not valid incoming direction"),
        },
        Ground => panic!("Ground is not a pipe"),
        Start => Direction::Up, // arbitrary direction
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        4
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        8
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        70
    )]
    fn test_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part1(input), expected);
    }
}
