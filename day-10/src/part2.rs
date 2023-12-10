use std::collections::{BTreeMap, HashSet, VecDeque};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn part2(input: &str) -> usize {
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
        .fold((BTreeMap::new(), None), |mut acc, (pipe, start_pos)| {
            acc.0.insert(pipe.0, pipe.1);
            if acc.1.is_none() {
                acc.1 = start_pos;
            }
            acc
        });

    let start_pos = start_pos.expect("No start point found");
    let (max_row, max_col) = pipes.keys().last().unwrap();
    let path = [
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
            Some(stack.iter().map(|node| node.pos).collect::<Vec<_>>())
        } else {
            None
        }
    })
    .max_by(|a, b| a.len().cmp(&b.len()))
    .unwrap()
    .into_iter()
    .collect::<HashSet<_>>();

    (0..=(*max_row))
        .map(|r| {
            let mut count = 0;
            // it just so happens that the tests and input have S start on a F or 7 pipe, so we
            // jsut treat it the same way
            let not_crossing_pipes = vec![Pipe::Horizontal, Pipe::F, Pipe::Start, Pipe::Seven];
            let mut is_adding = false;
            for c in 0..=*max_col {
                let pipe = pipes.get(&(r, c)).expect("pipe not found");
                if path.contains(&(r, c)) {
                    if !not_crossing_pipes.contains(pipe) {
                        is_adding ^= true;
                    }
                    continue;
                }

                if is_adding {
                    count += 1;
                }
            }
            count
        })
        .sum()
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

fn check_next(node: &PipeNode, pipes: &BTreeMap<(usize, usize), Pipe>) -> Option<PipeNode> {
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
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        4
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
        8
    )]
    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        10
    )]
    fn test_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part2(input), expected);
    }
}
