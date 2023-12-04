use color_eyre::eyre::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{char as nom_char, multispace1, u32 as nom_u32},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Game {
    _id: u32,
    winning_nums: Vec<u32>,
    played_nums: Vec<u32>,
}

impl Game {
    fn matches(&self) -> usize {
        self.played_nums
            .iter()
            .filter(|num| self.winning_nums.contains(num))
            .count()
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

fn parse(input: &str) -> IResult<&str, Game> {
    let (i, (_, _, id, _, _, (winning_nums, played_nums))) = tuple((
        tag("Card"),
        multispace1,
        nom_u32,
        nom_char(':'),
        multispace1,
        separated_pair(
            separated_list1(multispace1, nom_u32),
            tuple((multispace1, nom_char('|'), multispace1)),
            separated_list1(multispace1, nom_u32),
        ),
    ))(input)?;

    Ok((
        i,
        Game {
            _id: id,
            winning_nums,
            played_nums,
        },
    ))
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day4.txt");
    let games = input
        .lines()
        .map(|line| parse(line).map(|game| game.1))
        .collect::<Result<Vec<_>, _>>()?;

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
