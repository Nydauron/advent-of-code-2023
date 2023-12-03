use chumsky::prelude::*;

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

#[derive(PartialEq, Eq, Debug)]
struct Cubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

static MAX_CUBES: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};
static IMPOSSIBLE_GAME: u32 = 0;

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

fn part1_game(game_input: &str) -> u32 {
    let game = parser()
        .parse(game_input)
        .into_result()
        .expect("Parsing game failed");

    let and_reduce = |acc, x| acc & x;
    if game
        .rounds
        .iter()
        .map(|round| {
            round
                .colors
                .iter()
                .map(|color| match color {
                    Color::Red(amount) => *amount <= MAX_CUBES.red,
                    Color::Green(amount) => *amount <= MAX_CUBES.green,
                    Color::Blue(amount) => *amount <= MAX_CUBES.blue,
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
