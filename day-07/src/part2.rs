use std::collections::HashMap;

pub fn part2(input: &str) -> u32 {
    let hands = input.lines().map(parse_line).collect::<Vec<_>>();
    let mut ranks = hands
        .iter()
        .map(|hand| (hand, hand.get_rank()))
        .collect::<Vec<_>>();
    ranks.sort_by(|a, b| a.1.cmp(&b.1));

    ranks
        .iter()
        .enumerate()
        .map(|(idx, (hand, _))| hand.bid * (idx + 1) as u32)
        .sum()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    pub fn get_rank(&self) -> Rank {
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

        let mut card_count = card_count_map.into_iter().collect::<Vec<_>>();

        card_count.sort_by(|a, b| b.1.cmp(&a.1));
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
            (*most_cards).1 += number_of_jacks;
        } else {
            // card_count is empty because the hand was all full of Jacks
            card_count.push((&Card::Jack, number_of_jacks));
        }

        let mut rank = Rank {
            primary_strength: 0,
            subrank: self.cards.clone(),
        };
        for (idx, count_combo) in count_combos.iter().enumerate() {
            let does_match_combo = card_count
                .iter()
                .zip(count_combo)
                .map(|((_, count), combos)| count == combos)
                .fold(true, |acc, does_count_match| acc && does_count_match);
            if does_match_combo {
                rank.primary_strength = (count_combos.len() - idx) as u32;
                break;
            }
        }
        rank
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Ord)]
struct Rank {
    primary_strength: u32,
    subrank: Vec<Card>,
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.primary_strength != other.primary_strength {
            return Some(self.primary_strength.cmp(&other.primary_strength));
        }
        for (self_card, other_card) in self.subrank.iter().zip(other.subrank.iter()) {
            if self_card != other_card {
                return Some(self_card.cmp(other_card));
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
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

fn parse_line(line: &str) -> Hand {
    let (hand, bid_str) = line.split_once(" ").expect("Line did not parse");
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
