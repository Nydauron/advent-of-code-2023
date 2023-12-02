fn main() {
    let input = include_str!("input.txt");
    let games = input.lines();
    println!("{}", games.map(part2_game).sum::<u32>());
}

struct Cubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

static IMPOSSIBLE_GAME: u32 = 0;

fn part2_game(game: &str) -> u32 {
    if let Some((_, log)) = game.split_once(": ") {
        let rounds = log.split("; ");
        let mut max_cubes = Cubes{red: 0, green:0, blue: 0};
        for r in rounds {
            let cubes = r.split(", ");
            for c in cubes {
                if let Some((num_str, color)) = c.split_once(' ') {
                    let number = num_str.parse::<u32>().expect("Cube count was not a number");
                    match color {
                        "red" => {
                            if number > max_cubes.red {
                                max_cubes.red = number;
                            }
                        },
                        "green" => {
                            if number > max_cubes.green {
                                max_cubes.green = number;
                            }
                        },
                        "blue" => {
                            if number > max_cubes.blue {
                                max_cubes.blue = number;
                            }
                        },
                        _ => panic!("Non-valid color provided")
                    }
                } else {
                    panic!("Failed on game line: {}", game);
                }
            }
        }
        max_cubes.red * max_cubes.green * max_cubes.blue
    } else {
        IMPOSSIBLE_GAME
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;


    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    #[case("Game 6: 3 blue, 40 red; 1 red, 2 green, 6 blue; 2 green", 480)]
    #[case("Game 7: 36 blue, 20 red; 15 red, 18 green, 6 blue; 2 green", 12960)]
    fn test_part1_possible(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part2_game(input), expected)
    }
}
