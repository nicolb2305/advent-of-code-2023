use parse::parse;

mod parse {
    use crate::{Color, Draw, Game};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{space0, u32 as nom_u32},
        combinator::{map, value},
        multi::separated_list0,
        sequence::{separated_pair, tuple},
        IResult,
    };
    use std::convert::Into;

    fn draw(i: &str) -> IResult<&str, Draw> {
        map(
            separated_list0(
                tag(", "),
                separated_pair(
                    nom_u32,
                    space0,
                    alt((
                        value(Color::Red, tag("red")),
                        value(Color::Green, tag("green")),
                        value(Color::Blue, tag("blue")),
                    )),
                ),
            ),
            Into::into,
        )(i)
    }

    pub fn parse(i: &str) -> IResult<&str, Game> {
        let (_, (_, id, _, draws)) = tuple((
            tag("Game "),
            nom_u32,
            tag(": "),
            separated_list0(tag("; "), draw),
        ))(i)?;

        Ok((i, Game { id, draws }))
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
        .map(|line| parse(line).unwrap().1)
        .filter(|game| game.is_valid(&max_draw))
        .map(|game| game.id)
        .sum();
    println!("Sum of ids of valid games: {id_sum}");

    let power: u32 = input
        .lines()
        .map(|line| parse(line).unwrap().1.fewest_possible().power())
        .sum();
    println!("Sum of power of fewest possible cubes: {power}");
}
