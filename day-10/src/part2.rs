use std::collections::{BTreeMap, HashSet, VecDeque};

pub fn part2(input: &str) -> usize {
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
        .collect::<BTreeMap<_, _>>();

    let (max_row, max_col) = pipes.keys().last().unwrap();
    let start_pos = pipes
        .iter()
        .find_map(|(pos, row)| (*row == Pipe::Start).then_some(pos))
        .expect("No start pos found");

    let mut starting_pos = VecDeque::new();
    add_top(&mut starting_pos, start_pos);
    add_bottom(&mut starting_pos, start_pos);
    add_left(&mut starting_pos, start_pos);
    add_right(&mut starting_pos, start_pos);

    let path = starting_pos
        .iter()
        .filter_map(|&next_start| {
            let mut stack = VecDeque::from([next_start.clone()]);
            let mut visited = HashSet::new();
            let mut path = VecDeque::new();

            let mut found_cycle = false;
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
                            found_cycle = true;
                            break;
                        }
                    }
                }
            }
            if found_cycle {
                Some(path)
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
    pipes: &BTreeMap<(usize, usize), Pipe>,
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
    pipes: &BTreeMap<(usize, usize), Pipe>,
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
    pipes: &BTreeMap<(usize, usize), Pipe>,
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
    pipes: &BTreeMap<(usize, usize), Pipe>,
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
