pub fn part1(input: &str) -> i32 {
    let lines = input.lines();

    let readings = lines.map(|line| {
        line.split(" ")
            .map(|num_str| num_str.parse::<i32>().expect("Could not parse integer"))
            .collect::<Vec<_>>()
    });

    readings
        .map(|mut diff| {
            let mut last_elements = Vec::new();
            while !diff.iter().all(|d| *d == 0) {
                last_elements.push(diff.last().cloned().unwrap());
                diff = diff
                    .iter()
                    .map_windows(|[&x, &y]| y - x)
                    .collect::<Vec<_>>();
            }
            last_elements.iter().rfold(0, |acc, last| last + acc)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(part1(input), 114);
    }
}
