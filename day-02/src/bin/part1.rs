#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("input.txt");
    let games = input.lines();
    println!("{}", games.map(part1_game).sum::<u32>());
}

struct Cubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

static POSSIBLE_CUBES: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};
static IMPOSSIBLE_GAME: u32 = 0;

fn part1_game(game: &str) -> u32 {
    if let Some((id, log)) = game.split_once(": ") {
        let id = id
            .strip_prefix("Game ")
            .unwrap()
            .parse::<u32>()
            .expect("Not a valid game id");
        let rounds = log.split("; ");
        for r in rounds {
            let cubes = r.split(", ");
            for c in cubes {
                if let Some((num_str, color)) = c.split_once(' ') {
                    let number = num_str.parse::<u32>().expect("Cube count was not a number");
                    match color {
                        "red" => {
                            if number > POSSIBLE_CUBES.red {
                                return IMPOSSIBLE_GAME;
                            }
                        }
                        "green" => {
                            if number > POSSIBLE_CUBES.green {
                                return IMPOSSIBLE_GAME;
                            }
                        }
                        "blue" => {
                            if number > POSSIBLE_CUBES.blue {
                                return IMPOSSIBLE_GAME;
                            }
                        }
                        _ => panic!("Non-valid color provided"),
                    }
                } else {
                    panic!("Failed on game line: {}", game);
                }
            }
        }
        id
    } else {
        IMPOSSIBLE_GAME
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1)]
    #[case("Game 1: 3 blue, 40 red; 1 red, 2 green, 6 blue; 2 green", 0)]
    fn test_part1_possible(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part1_game(input), expected)
    }
}
