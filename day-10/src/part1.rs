use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(input: &str) -> usize {
    let pipes = input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line.chars()
                .map(|c| {
                    use Pipe::*;
                    match c {
                        '|' => Vertical,
                        '-' => Horizontal,
                        'L' => L,
                        'J' => J,
                        '7' => Seven,
                        'F' => F,
                        '.' => Ground,
                        'S' => Start,
                        c => panic!("Unexpected char found: {}", c),
                    }
                })
                .enumerate()
                .map(move |(col_idx, pipe)| ((row_idx, col_idx), pipe))
        })
        .collect::<HashMap<_, _>>();

    let start_pos = pipes
        .iter()
        .find_map(|(pos, row)| (*row == Pipe::Start).then_some(pos))
        .expect("No start pos found");

    let mut starting_pos = VecDeque::new();
    add_top(&mut starting_pos, start_pos);
    add_bottom(&mut starting_pos, start_pos);
    add_left(&mut starting_pos, start_pos);
    add_right(&mut starting_pos, start_pos);

    starting_pos
        .iter()
        .filter_map(|&next_start| {
            let mut stack = VecDeque::from([next_start.clone()]);
            let mut visited = HashSet::new();
            let mut path = VecDeque::new();

            while !stack.is_empty() {
                if let Some(curr) = stack.pop_back() {
                    if let Some(path_end) = path.back() {
                        if *path_end == curr {
                            path.pop_back();
                            continue;
                        }
                    }
                    if visited.contains(&curr) {
                        continue;
                    }
                    path.push_back(curr);
                    stack.push_back(curr);
                    visited.insert(curr);
                    if let Some(pipe) = pipes.get(&curr) {
                        use Pipe::*;
                        if match pipe {
                            Vertical => {
                                check_top(&curr, &pipes, &mut stack);
                                check_bottom(&curr, &pipes, &mut stack);
                                false
                            }
                            Horizontal => {
                                check_left(&curr, &pipes, &mut stack);
                                check_right(&curr, &pipes, &mut stack);
                                false
                            }
                            L => {
                                check_top(&curr, &pipes, &mut stack);
                                check_right(&curr, &pipes, &mut stack);
                                false
                            }
                            J => {
                                check_top(&curr, &pipes, &mut stack);
                                check_left(&curr, &pipes, &mut stack);
                                false
                            }
                            Seven => {
                                check_left(&curr, &pipes, &mut stack);
                                check_bottom(&curr, &pipes, &mut stack);
                                false
                            }
                            F => {
                                check_right(&curr, &pipes, &mut stack);
                                check_bottom(&curr, &pipes, &mut stack);
                                false
                            }
                            Start => true,
                            Ground => false,
                        } {
                            break;
                        }
                    }
                }
            }
            if path.len() != 0 {
                Some(path)
            } else {
                None
            }
        })
        .map(|path| path.len())
        .max()
        .unwrap()
        / 2
}

fn add_top(queue: &mut VecDeque<(usize, usize)>, current_position: &(usize, usize)) {
    if current_position.0 == 0 {
        return;
    }
    let top_position = (current_position.0 - 1, current_position.1);
    queue.push_back(top_position);
}

fn add_bottom(queue: &mut VecDeque<(usize, usize)>, current_position: &(usize, usize)) {
    let bottom_position = (current_position.0 + 1, current_position.1);
    queue.push_back(bottom_position);
}
fn add_left(queue: &mut VecDeque<(usize, usize)>, current_position: &(usize, usize)) {
    if current_position.1 == 0 {
        return;
    }
    let left_position = (current_position.0, current_position.1 - 1);
    queue.push_back(left_position);
}
fn add_right(queue: &mut VecDeque<(usize, usize)>, current_position: &(usize, usize)) {
    let right_position = (current_position.0, current_position.1 + 1);
    queue.push_back(right_position);
}
fn check_top(
    position: &(usize, usize),
    pipes: &HashMap<(usize, usize), Pipe>,
    queue: &mut VecDeque<(usize, usize)>,
) -> bool {
    use Pipe::*;
    let top_position = (if position.0 > 0 { position.0 - 1 } else { 0 }, position.1);
    let acceptable_pipe_types = vec![Vertical, Seven, F, Start];
    pipes
        .get(&top_position)
        .and_then(|pipe_type| {
            if acceptable_pipe_types.contains(pipe_type) {
                add_top(queue, position);
                Some(true)
            } else {
                None
            }
        })
        .unwrap_or(false)
}

fn check_bottom(
    position: &(usize, usize),
    pipes: &HashMap<(usize, usize), Pipe>,
    queue: &mut VecDeque<(usize, usize)>,
) -> bool {
    use Pipe::*;
    let bottom_position = (position.0 + 1, position.1);
    let acceptable_pipe_types = vec![Vertical, L, J, Start];
    pipes
        .get(&bottom_position)
        .and_then(|pipe_type| {
            if acceptable_pipe_types.contains(pipe_type) {
                add_bottom(queue, position);
                Some(true)
            } else {
                None
            }
        })
        .unwrap_or(false)
}

fn check_left(
    position: &(usize, usize),
    pipes: &HashMap<(usize, usize), Pipe>,
    queue: &mut VecDeque<(usize, usize)>,
) -> bool {
    use Pipe::*;
    let left_position = (position.0, if position.1 > 0 { position.1 - 1 } else { 0 });
    let acceptable_pipe_types = vec![Horizontal, L, F, Start];
    pipes
        .get(&left_position)
        .and_then(|pipe_type| {
            if acceptable_pipe_types.contains(pipe_type) {
                add_left(queue, position);
                Some(true)
            } else {
                None
            }
        })
        .unwrap_or(false)
}
fn check_right(
    position: &(usize, usize),
    pipes: &HashMap<(usize, usize), Pipe>,
    queue: &mut VecDeque<(usize, usize)>,
) -> bool {
    use Pipe::*;
    let right_position = (position.0, position.1 + 1);
    let acceptable_pipe_types = vec![Horizontal, Seven, J, Start];
    pipes
        .get(&right_position)
        .and_then(|pipe_type| {
            if acceptable_pipe_types.contains(pipe_type) {
                add_right(queue, position);
                Some(true)
            } else {
                None
            }
        })
        .unwrap_or(false)
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
