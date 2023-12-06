pub fn part2(input: &str) -> u64 {
    let (times, distances) = input.split_once("\n").expect("could not parse correctly");
    let time = times
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .expect("time did not parse");
    let distance = distances
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .expect("distance did not parse");

    let lower_bound = find_lowest_time_possible(0, time / 2 + 1, time, distance);
    time - lower_bound * 2 + 1
}

fn find_lowest_time_possible(t_lower: u64, t_upper: u64, total_time: u64, distance: u64) -> u64 {
    if t_lower >= t_upper {
        t_lower
    } else {
        let m = (t_lower + t_upper) / 2;
        let distance_traveled = (total_time - m) * m;
        if distance_traveled <= distance {
            find_lowest_time_possible(m + 1, t_upper, total_time, distance)
        } else {
            find_lowest_time_possible(t_lower, m, total_time, distance)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part2(input), 71503);
    }
}
