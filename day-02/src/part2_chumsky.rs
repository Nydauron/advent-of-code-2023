use chumsky::prelude::*;

pub fn part2(input: &str) -> u32 {
    let games = input.lines();
    games.map(part2_game).sum::<u32>()
}

#[derive(PartialEq, Eq, Debug)]
struct Cubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(PartialEq, Debug, Clone)]
struct Round {
    pub colors: Vec<Color>,
}

#[derive(PartialEq, Debug, Clone)]
struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

fn parser<'a>() -> impl Parser<'a, &'a str, Game, extra::Err<Rich<'a, char>>> {
    let game_id = just("Game ")
        .ignore_then(text::int(10))
        .from_str::<u32>()
        .unwrapped();

    let color = text::int(10)
        .padded()
        .from_str::<u32>()
        .unwrapped()
        .then(text::ascii::ident())
        .try_map(|(amount, name), span| match name {
            "red" => Ok(Color::Red(amount)),
            "blue" => Ok(Color::Blue(amount)),
            "green" => Ok(Color::Green(amount)),
            _ => Err(Rich::custom(span, format!("{} is not a valid color", name))),
        });
    let round = color
        .separated_by(just(','))
        .collect::<Vec<_>>()
        .map(|a| Round { colors: a });
    game_id
        .then_ignore(just(": "))
        .then(round.padded().separated_by(just(';')).collect::<Vec<_>>())
        .map(|(id, rounds)| Game { id, rounds })
}

fn part2_game(game_input: &str) -> u32 {
    let game = parser()
        .parse(game_input)
        .into_result()
        .expect("Parsing game failed");

    let powers = game
        .rounds
        .iter()
        .map(|round| {
            round.colors.iter().fold(
                Cubes {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut acc, color| {
                    match color {
                        Color::Red(amount) => acc.red += amount,
                        Color::Blue(amount) => acc.blue += amount,
                        Color::Green(amount) => acc.green += amount,
                    }
                    acc
                },
            )
        })
        .fold(
            Cubes {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut acc, round_totals| {
                acc.red = acc.red.max(round_totals.red);
                acc.green = acc.green.max(round_totals.green);
                acc.blue = acc.blue.max(round_totals.blue);
                acc
            },
        );
    powers.red * powers.green * powers.blue
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    #[case("Game 6: 3 blue, 40 red; 1 red, 2 green, 6 blue; 2 green", 480)]
    #[case("Game 7: 36 blue, 20 red; 15 red, 18 green, 6 blue; 2 green", 12960)]
    fn test_part1_possible(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part2_game(input), expected)
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        Game {
            id: 1,
            rounds: vec![
                Round{
                    colors: vec![
                        Color::Blue(3),
                        Color::Red(4),
                    ]
                },
                Round{
                    colors: vec![
                        Color::Red(1),
                        Color::Green(2),
                        Color::Blue(6),
                    ]
                },
                Round{
                    colors: vec![
                        Color::Green(2),
                    ]
                }
            ]
        }
    )]
    fn test_parser(#[case] input: &str, #[case] expected: Game) {
        assert_eq!(parser().parse(input).into_result(), Ok(expected))
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 purple, 6 blue; 2 green")]
    fn test_parser_failure(#[case] input: &str) {
        assert!(parser().parse(input).into_result().is_err());
    }
}
