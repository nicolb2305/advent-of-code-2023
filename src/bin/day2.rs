use parse::parse;

mod parse {
    use crate::{Color, Draw, Game};
    use anyhow::{anyhow, Result};
    use std::convert::Into;
    use winnow::{
        ascii::{dec_uint, space0},
        combinator::{alt, separated, separated_pair},
        prelude::*,
    };

    fn draw(i: &mut &str) -> PResult<Draw> {
        separated(
            0..,
            separated_pair(
                dec_uint,
                space0,
                alt((
                    "red".value(Color::Red),
                    "green".value(Color::Green),
                    "blue".value(Color::Blue),
                )),
            ),
            ", ",
        )
        .map(|x: Vec<(u32, Color)>| x.into())
        .parse_next(i)
    }

    fn parser(i: &mut &str) -> PResult<Game> {
        ("Game ", dec_uint, ": ", separated(0.., draw, "; "))
            .map(|(_, id, _, draws)| Game { id, draws })
            .parse_next(i)
    }

    pub fn parse(i: &str) -> Result<Game> {
        parser.parse(i).map_err(|e| anyhow!(e.to_string()))
    }
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Default, Clone, Copy)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<Vec<(u32, Color)>> for Draw {
    fn from(value: Vec<(u32, Color)>) -> Self {
        let mut draw = Draw::default();
        for (count, color) in value {
            match color {
                Color::Red => draw.red = count,
                Color::Green => draw.green += count,
                Color::Blue => draw.blue += count,
            }
        }
        draw
    }
}

impl Draw {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn is_valid(&self, max_draw: &Draw) -> bool {
        self.draws.iter().all(|draw| {
            draw.red <= max_draw.red && draw.green <= max_draw.green && draw.blue <= max_draw.blue
        })
    }

    fn fewest_possible(&self) -> Draw {
        self.draws.iter().fold(Draw::default(), |mut acc, x| {
            acc.red = acc.red.max(x.red);
            acc.green = acc.green.max(x.green);
            acc.blue = acc.blue.max(x.blue);
            acc
        })
    }
}

fn main() {
    let input = include_str!("../../input/day2.txt");

    let max_draw = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };
    let id_sum: u32 = input
        .lines()
        .flat_map(parse)
        .filter(|game| game.is_valid(&max_draw))
        .map(|game| game.id)
        .sum();
    println!("Sum of ids of valid games: {id_sum}");

    let power: u32 = input
        .lines()
        .flat_map(parse)
        .map(|game| game.fewest_possible().power())
        .sum();
    println!("Sum of power of fewest possible cubes: {power}");
}
