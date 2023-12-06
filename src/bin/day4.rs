use color_eyre::{eyre::eyre, Result};
use std::collections::HashSet;
use winnow::{
    ascii::{dec_uint, multispace1},
    combinator::{separated, separated_pair},
    prelude::*,
};

#[derive(Debug)]
struct Game {
    _id: u32,
    winning_nums: HashSet<u32>,
    played_nums: HashSet<u32>,
}

impl Game {
    fn matches(&self) -> usize {
        self.played_nums.intersection(&self.winning_nums).count()
    }

    fn score(&self) -> u32 {
        let count = self.matches();
        if count == 0 {
            0
        } else {
            1 << (count - 1)
        }
    }
}

fn parser(input: &mut &str) -> PResult<Game> {
    let (_, _, id, _, _, (winning_nums, played_nums)) = (
        "Card",
        multispace1,
        dec_uint,
        ':',
        multispace1,
        separated_pair(
            separated(1.., dec_uint, multispace1),
            (multispace1, '|', multispace1),
            separated(1.., dec_uint, multispace1),
        ),
    )
        .parse_next(input)?;

    Ok(Game {
        _id: id,
        winning_nums,
        played_nums,
    })
}

fn parse(i: &str) -> Result<Game> {
    parser.parse(i).map_err(|e| eyre!(e.to_string()))
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day4.txt");
    let games = input.lines().map(parse).collect::<Result<Vec<_>, _>>()?;

    let score: u32 = games.iter().map(Game::score).sum();
    println!("Total points: {score}");

    let mut repeats = vec![1; games.len()];
    for (i, game) in games.iter().enumerate() {
        for j in 1..=game.matches() {
            let num_added = repeats[i];
            match repeats.get_mut(i + j) {
                Some(v) => *v += num_added,
                None => continue,
            }
        }
    }
    let num_scratchcards: u32 = repeats.iter().sum();
    println!("Total number of scratchcards: {num_scratchcards}");
    Ok(())
}
