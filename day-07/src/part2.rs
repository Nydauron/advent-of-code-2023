use std::collections::HashMap;

use num_enum::IntoPrimitive;

pub fn part2(input: &str) -> u32 {
    let mut ranks = input
        .lines()
        .map(|line| parse_line(line).get_rank())
        .collect::<Vec<_>>();
    ranks.sort();

    ranks
        .iter()
        .enumerate()
        .map(|(idx, rank)| rank.bid * (idx + 1) as u32)
        .sum()
}

#[derive(Debug, Clone, Copy, Hash, IntoPrimitive, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err("Character cannot be parsed"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    pub fn get_rank(self) -> HandRank {
        let mut card_count_map =
            self.cards
                .iter()
                .fold(HashMap::<&Card, u32>::new(), |mut acc, card| {
                    acc.entry(card)
                        .and_modify(|a| {
                            *a += 1;
                        })
                        .or_insert(1);
                    acc
                });
        let number_of_jacks = card_count_map.remove(&Card::Jack).unwrap_or(0);

        let mut card_count = card_count_map
            .values()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        card_count.sort_by(|a, b| b.cmp(&a));
        let count_combos = [
            Vec::from([5]),
            Vec::from([4]),
            Vec::from([3, 2]),
            Vec::from([3]),
            Vec::from([2, 2]),
            Vec::from([2]),
        ];
        let most_cards = card_count.get_mut(0);
        if let Some(most_cards) = most_cards {
            *most_cards += number_of_jacks;
        } else {
            // card_count is empty because the hand was all full of Jacks
            card_count.push(number_of_jacks);
        }

        let mut primary_strength = 0;
        for (idx, count_combo) in count_combos.iter().enumerate() {
            let does_match_combo = card_count
                .iter()
                .zip(count_combo)
                .map(|(count, combos)| count == combos)
                .all(|does_count_match| does_count_match);
            if does_match_combo {
                primary_strength = (count_combos.len() - idx) as u32;
                break;
            }
        }

        let secondary_strength = self
            .cards
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, card)| {
                let card_val: u32 = (*card).into();
                16_u32.pow(idx as u32) * card_val
            })
            .sum();

        HandRank {
            primary_strength,
            secondary_strength,
            bid: self.bid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HandRank {
    primary_strength: u32,
    secondary_strength: u32,
    bid: u32,
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.primary_strength != other.primary_strength {
            self.primary_strength.cmp(&other.primary_strength)
        } else {
            self.secondary_strength.cmp(&other.secondary_strength)
        }
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_line(line: &str) -> Hand {
    let (hand, bid_str) = line.split_once(' ').expect("Line did not parse");
    let cards = hand
        .chars()
        .map(|card| Card::try_from(card).expect("Did not parse character"))
        .collect::<Vec<_>>();
    let bid_amt = bid_str.parse::<u32>().expect("Did not parse bid amount");
    Hand {
        cards,
        bid: bid_amt,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        assert_eq!(part2(input), 5905);
    }
}
