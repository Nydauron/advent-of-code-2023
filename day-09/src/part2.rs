pub fn part2(input: &str) -> i32 {
    let lines = input.lines();

    let readings = lines.map(|line| {
        line.split(" ")
            .map(|num_str| num_str.parse::<i32>().expect("Could not parse integer"))
            .collect::<Vec<_>>()
    });

    readings
        .map(|mut diff| {
            let mut first_elements = Vec::new();
            while !diff.iter().all(|d| *d == 0) {
                first_elements.push(diff.first().cloned().unwrap());
                diff = diff
                    .iter()
                    .map_windows(|[&x, &y]| y - x)
                    .collect::<Vec<_>>();
            }
            first_elements.iter().rfold(0, |acc, first| first - acc)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(part2(input), 2);
    }
}
