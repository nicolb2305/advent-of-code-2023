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

impl Card {
    fn joker_cmp(self, other: Self) -> Ordering {
        match (self, other) {
            (Self::J, Self::J) => Ordering::Equal,
            (Self::J, _) => Ordering::Less,
            (_, Self::J) => Ordering::Greater,
            (x, y) => x.cmp(&y),
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

pub trait Hand {
    fn number_of_a_kind_with_exclusion(&self, num: usize, exclude: Option<Card>) -> Option<Card>;
    fn number_of_a_kind(&self, num: usize) -> Option<Card> {
        self.number_of_a_kind_with_exclusion(num, None)
    }
    fn full_house(&self) -> Option<HandType>;
    fn two_pair(&self) -> Option<HandType>;
    fn hand_type(&self) -> HandType;
    fn bid(&self) -> u64;
}

#[derive(Debug, PartialEq, Eq)]
pub struct NoJokerHand {
    cards: Vec<Card>,
    bid: u64,
}

impl Hand for NoJokerHand {
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

    fn bid(&self) -> u64 {
        self.bid
    }
}

impl PartialOrd for NoJokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NoJokerHand {
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

#[derive(Debug, PartialEq, Eq)]
pub struct JokerHand {
    cards: Vec<Card>,
    bid: u64,
}

impl Hand for JokerHand {
    fn number_of_a_kind_with_exclusion(&self, num: usize, exclude: Option<Card>) -> Option<Card> {
        Card::iter()
            .filter(|&card| {
                if let Some(exclude) = exclude {
                    card != exclude
                } else {
                    true
                }
            })
            .find(|card_type| {
                self.cards
                    .iter()
                    .filter(|&card| card == card_type || card == &Card::J)
                    .count()
                    == num
            })
    }

    fn full_house(&self) -> Option<HandType> {
        let three_of_a_kind = Card::iter().find_map(|card_type| {
            let locs: Vec<_> = self
                .cards
                .iter()
                .enumerate()
                .filter_map(|(i, card)| {
                    if card == &card_type || card == &Card::J {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();
            if locs.len() >= 3 {
                Some(locs)
            } else {
                None
            }
        })?;
        // let card_type = self.cards[three_of_a_kind[0]];
        let _ = Card::iter().find(|&card_type| {
            self.cards
                .iter()
                .enumerate()
                .filter(|(i, &card)| {
                    !three_of_a_kind.contains(i) && (card == card_type || card == Card::J)
                })
                .count()
                == 2
        })?;
        Some(HandType::FullHouse)
    }

    fn two_pair(&self) -> Option<HandType> {
        let two_of_a_kind = Card::iter().find_map(|card_type| {
            let locs: Vec<_> = self
                .cards
                .iter()
                .enumerate()
                .filter_map(|(i, card)| {
                    if card == &card_type || card == &Card::J {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();
            if locs.len() >= 2 {
                Some(locs)
            } else {
                None
            }
        })?;
        let _ = Card::iter().find(|&card_type| {
            self.cards
                .iter()
                .enumerate()
                .filter(|(i, &card)| {
                    !two_of_a_kind.contains(i) && (card == card_type || card == Card::J)
                })
                .count()
                == 2
        })?;
        Some(HandType::TwoPair)
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

    fn bid(&self) -> u64 {
        self.bid
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = self.hand_type().cmp(&other.hand_type());
        if Ordering::Equal == order {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(x, y)| match x.joker_cmp(*y) {
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
    use crate::{Card, DayResult, Error, Game, NoJokerHand};
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

    fn hand(input: &mut &str) -> PResult<NoJokerHand> {
        let (cards, _, bid) = (repeat(1.., card), multispace1, dec_uint).parse_next(input)?;
        Ok(NoJokerHand { cards, bid })
    }

    fn game(input: &mut &str) -> PResult<Game<NoJokerHand>> {
        let hands = separated(1.., hand, multispace1).parse_next(input)?;
        Ok(Game { hands })
    }

    pub fn parse(input: &str) -> DayResult<Game<NoJokerHand>> {
        game.parse(input)
            .map_err(|e| Error::LineParsingError(e.to_string()))
    }
}

#[derive(Debug)]
pub struct Game<H> {
    hands: Vec<H>,
}

impl<H: Hand + Ord> Game<H> {
    fn winnings(&mut self) -> u64 {
        self.hands.sort_unstable();
        self.hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i as u64 + 1) * hand.bid())
            .sum()
    }
}

impl Game<NoJokerHand> {
    fn convert(self) -> Game<JokerHand> {
        let hands = self
            .hands
            .into_iter()
            .map(|hand| JokerHand {
                cards: hand.cards,
                bid: hand.bid,
            })
            .collect();
        Game { hands }
    }
}

fn main() -> DayResult<()> {
    let input = include_str!("../../input/day7.txt");
    let mut game = parse(input)?;
    let winnings = game.winnings();
    println!("Winnings: {winnings}");
    // assert_eq!(winnings, 253_910_319);

    let mut game2 = game.convert();
    let winnings_with_joker = game2.winnings();
    println!("Winnings with joker: {winnings_with_joker}");
    // assert_eq!(winnings_with_joker, 254_083_736);
    Ok(())
}
