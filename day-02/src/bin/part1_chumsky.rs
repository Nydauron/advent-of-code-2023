use std::collections::HashMap;

use chumsky::{primitive::just, text, IterParser, Parser};

fn main() {
    let input = include_str!("input.txt");
    let games = input.lines();
    println!("{}", games.map(part1_game).sum::<u32>());
}

static IMPOSSIBLE_GAME: u32 = 0;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Color<'a> {
    pub name: &'a str,
    pub amount: u32,
}

#[derive(PartialEq, Debug, Clone)]
struct Round<'a> {
    pub colors: Vec<Color<'a>>,
}

#[derive(PartialEq, Debug, Clone)]
struct Game<'a> {
    pub id: u32,
    pub rounds: Vec<Round<'a>>,
}

fn parser<'a>() -> impl Parser<'a, &'a str, Game<'a>> {
    let game_id = just("Game ")
        .ignore_then(text::int(10))
        .from_str::<u32>()
        .unwrapped();

    let color = text::int(10)
        .padded()
        .from_str::<u32>()
        .unwrapped()
        .then(text::ascii::ident())
        .map(|(amount, name)| Color { name, amount });
    let round = color
        .separated_by(just(','))
        .collect::<Vec<_>>()
        .map(|a| Round { colors: a });
    game_id
        .then_ignore(just(": "))
        .then(round.padded().separated_by(just(';')).collect::<Vec<_>>())
        .map(|(id, rounds)| Game { id, rounds })
}

fn part1_game(game_input: &str) -> u32 {
    let possible_cubes: HashMap<&str, u32> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let game = parser()
        .parse(game_input)
        .into_result()
        .expect("Parsing game failed");

    let and_reduce = |acc, x| return acc & x;
    if game
        .rounds
        .iter()
        .map(|round| {
            round
                .colors
                .iter()
                .map(|color| {
                    let max_color_allowed =
                        possible_cubes.get(color.name).expect("invlid color found");
                    color.amount <= *max_color_allowed
                })
                .fold(true, and_reduce)
        })
        .fold(true, and_reduce)
    {
        game.id
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

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 
        Game {
            id: 1,
            rounds: vec![
                Round{
                    colors: vec![
                        Color{
                            name: "blue",
                            amount: 3
                        },
                        Color{
                            name: "red",
                            amount: 4,
                        }
                    ]
                },
                Round{
                    colors: vec![
                        Color{
                            name: "red",
                            amount: 1,
                        },
                        Color{
                            name: "green",
                            amount: 2,
                        },
                        Color{
                            name: "blue",
                            amount: 6,
                        },
                    ]
                },
                Round{
                    colors: vec![
                        Color{
                            name: "green",
                            amount: 2,
                        }
                    ]
                }
            ]
        }
    )]
    fn test_parser(#[case] input: &str, #[case] expected: Game) {
        assert_eq!(parser().parse(input).into_result(), Ok(expected))
    }
}
