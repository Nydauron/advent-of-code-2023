pub fn part1(input: &str) -> u32 {
    let (times, distances) = input.split_once("\n").expect("could not parse correctly");
    let times = times
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|t| t.parse::<u32>().expect("time did not parse"));
    let distances = distances
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|d| d.parse::<u32>().expect("distance did not parse"));

    times
        .zip(distances)
        .map(|(t, d)| {
            let lower_bound = find_lowest_time_possible(0, t / 2 + 1, t, d);
            t - lower_bound * 2 + 1
        })
        .product()
}

fn find_lowest_time_possible(t_lower: u32, t_upper: u32, total_time: u32, distance: u32) -> u32 {
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
    fn test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(input), 288);
    }
}
