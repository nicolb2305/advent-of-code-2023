use parse::parse;
use std::{cmp::Ordering, collections::HashMap, str::FromStr};
use strum::{EnumIter, IntoEnumIterator};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to parse {0}")]
    CardParseError(String),
    #[error("failed to parse card line {0}")]
    LineParsingError(String),
}

pub type DayResult<T> = Result<T, Error>;
#[derive(Debug, PartialEq, Eq, EnumIter, Clone, Copy, Hash)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn cmp(self, other: Self, no_joker: bool) -> Ordering {
        let compare = (self as u32).cmp(&(other as u32));
        if no_joker {
            compare
        } else {
            match (self, other) {
                (Self::J, Self::J) => Ordering::Equal,
                (Self::J, _) => Ordering::Less,
                (_, Self::J) => Ordering::Greater,
                _ => compare,
            }
        }
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::A,
            "K" => Self::K,
            "Q" => Self::Q,
            "J" => Self::J,
            "T" => Self::T,
            "9" => Self::Nine,
            "8" => Self::Eight,
            "7" => Self::Seven,
            "6" => Self::Six,
            "5" => Self::Five,
            "4" => Self::Four,
            "3" => Self::Three,
            "2" => Self::Two,
            x => return Err(Error::CardParseError(x.to_owned())),
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

impl Hand {
    fn hand_type(&self, no_joker: bool) -> HandType {
        let mut map: HashMap<_, _> = Card::iter().map(|card| (card, 0)).collect();
        for card in &self.cards {
            *map.get_mut(card).unwrap() += 1;
        }
        let j_count = if no_joker {
            0
        } else {
            map.remove(&Card::J).unwrap_or(0)
        };
        let max_card = *map.iter().max_by_key(|(_, &i)| i).unwrap().0;
        let max_count = map.remove(&max_card).unwrap_or(0);
        let second_max_count = map.values().max().copied().unwrap_or(0);

        match (max_count + j_count, second_max_count) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn cmp(&self, other: &Self, no_joker: bool) -> Ordering {
        self.hand_type(no_joker)
            .cmp(&other.hand_type(no_joker))
            .then_with(|| {
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .fold(Ordering::Equal, |order, (x, &y)| {
                        order.then_with(|| x.cmp(y, no_joker))
                    })
            })
    }
}

#[derive(Debug)]
pub struct Game {
    hands: Vec<Hand>,
}

impl Game {
    fn winnings(&mut self, no_joker: bool) -> u64 {
        self.hands.sort_by(|x, y| x.cmp(y, no_joker));
        self.hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i as u64 + 1) * hand.bid)
            .sum()
    }
}

mod parse {
    use crate::{Card, DayResult, Error, Game, Hand};
    use std::str::FromStr;
    use winnow::{
        ascii::{dec_uint, multispace1},
        combinator::{repeat, separated},
        prelude::*,
        token::take,
    };

    fn card(input: &mut &str) -> PResult<Card> {
        take(1usize).try_map(Card::from_str).parse_next(input)
    }

    fn hand(input: &mut &str) -> PResult<Hand> {
        let (cards, _, bid) = (repeat(1.., card), multispace1, dec_uint).parse_next(input)?;
        Ok(Hand { cards, bid })
    }

    fn game(input: &mut &str) -> PResult<Game> {
        let hands = separated(1.., hand, multispace1).parse_next(input)?;
        Ok(Game { hands })
    }

    pub fn parse(input: &str) -> DayResult<Game> {
        game.parse(input)
            .map_err(|e| Error::LineParsingError(e.to_string()))
    }
}

fn main() -> DayResult<()> {
    let input = include_str!("../../input/day7.txt");
    let mut game = parse(input)?;
    let winnings = game.winnings(true);
    println!("Winnings: {winnings}");
    assert_eq!(winnings, 253_910_319);

    let winnings_with_joker = game.winnings(false);
    println!("Winnings with joker: {winnings_with_joker}");
    assert_eq!(winnings_with_joker, 254_083_736);

    Ok(())
}
