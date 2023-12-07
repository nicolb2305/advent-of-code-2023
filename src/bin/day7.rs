use parse::parse;
use std::{cmp::Ordering, str::FromStr};
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, Clone, Copy)]
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
    fn number_of_a_kind_with_exclusion(&self, num: usize, exclude: Option<Card>) -> Option<Card> {
        Card::iter()
            .filter(|&card| {
                if let Some(exclude) = exclude {
                    card != exclude
                } else {
                    true
                }
            })
            .find(|card_type| self.cards.iter().filter(|&card| card == card_type).count() == num)
    }

    fn number_of_a_kind(&self, num: usize) -> Option<Card> {
        self.number_of_a_kind_with_exclusion(num, None)
    }

    fn full_house(&self) -> Option<HandType> {
        self.number_of_a_kind(3)
            .and_then(|_| self.number_of_a_kind(2))
            .map(|_| HandType::FullHouse)
    }

    fn two_pair(&self) -> Option<HandType> {
        self.number_of_a_kind(2)
            .and_then(|card1| self.number_of_a_kind_with_exclusion(2, Some(card1)))
            .map(|_| HandType::TwoPair)
    }

    fn hand_type(&self) -> HandType {
        if self.number_of_a_kind(5).is_some() {
            return HandType::FiveOfAKind;
        }

        if self.number_of_a_kind(4).is_some() {
            return HandType::FourOfAKind;
        }

        if let Some(hand_type) = self.full_house() {
            return hand_type;
        }

        if self.number_of_a_kind(3).is_some() {
            return HandType::ThreeOfAKind;
        }

        if let Some(hand_type) = self.two_pair() {
            return hand_type;
        }

        if self.number_of_a_kind(2).is_some() {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = self.hand_type().cmp(&other.hand_type());
        if Ordering::Equal == order {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(x, y)| match x.cmp(y) {
                    Ordering::Equal => None,
                    x => Some(x),
                })
                .unwrap_or(Ordering::Equal)
        } else {
            order
        }
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

#[derive(Debug)]
pub struct Game {
    hands: Vec<Hand>,
}

impl Game {
    fn winnings(&mut self) -> u64 {
        self.hands.sort_unstable();
        self.hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i as u64 + 1) * hand.bid)
            .sum()
    }
}

fn main() -> DayResult<()> {
    let input = include_str!("../../input/day7.txt");
    let mut game = parse(input)?;
    dbg!(&game);
    dbg!(game.winnings());
    Ok(())
}
