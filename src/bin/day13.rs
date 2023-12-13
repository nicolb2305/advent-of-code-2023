use advent_of_code_2023::coordinates::Grid;
use anyhow::Result;
use itertools::Itertools;
use parse::parse;
use std::fmt::Display;

mod parse {
    use crate::{Floor, Tile};
    use advent_of_code_2023::coordinates::Grid;
    use anyhow::{anyhow, Result};
    use winnow::{
        ascii::line_ending,
        combinator::{alt, repeat, separated},
        prelude::*,
    };

    fn row(input: &mut &str) -> PResult<Vec<Tile>> {
        repeat(1.., alt(('.'.value(Tile::Ash), '#'.value(Tile::Rocks)))).parse_next(input)
    }

    fn floor(input: &mut &str) -> PResult<Floor> {
        separated(1.., row, line_ending)
            .map(Grid)
            .map(Floor)
            .parse_next(input)
    }

    fn floors(input: &mut &str) -> PResult<Vec<Floor>> {
        separated(1.., floor, (line_ending, line_ending)).parse_next(input)
    }

    pub fn parse(input: &str) -> Result<Vec<Floor>> {
        floors.parse(input).map_err(|e| anyhow!(e.to_string()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rocks,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ash => '.',
                Self::Rocks => '#',
            }
        )
    }
}

#[derive(Debug)]
struct Floor(Grid<Tile>);

impl Floor {
    fn rotate(&self) -> Self {
        Self(Grid(
            (0..self.0 .0.first().unwrap().len())
                .map(|i| self.0 .0.iter().map(|row| row[i]).collect())
                .collect(),
        ))
    }

    fn mirror_value(&self, smudged: bool) -> usize {
        self.horizontal_mirror_rows(smudged)
            .map(|x| x * 100)
            .or(self.rotate().horizontal_mirror_rows(smudged))
            .expect("Failed to find any mirrored planes")
    }

    fn horizontal_mirror_rows(&self, smudged: bool) -> Option<usize> {
        (1..self.0 .0.len()).find(|&i| {
            let mut lower_index = i;
            let mut upper_index = i - 1;
            let mut smudged = !smudged;
            loop {
                let (Some(upper), Some(lower)) =
                    (self.0 .0.get(upper_index), self.0 .0.get(lower_index))
                else {
                    return smudged;
                };

                match upper.iter().zip(lower).filter(|(x, y)| x != y).count() {
                    0 => {}
                    1 if !smudged => smudged = true,
                    _ => return false,
                }

                upper_index = match upper_index.checked_sub(1) {
                    Some(i) => i,
                    None => return smudged,
                };
                lower_index += 1;
            }
        })
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                 .0
                .iter()
                .map(|row| row.iter().map(ToString::to_string).collect::<String>())
                .join("\n")
        )
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day13.txt");
    let floors = parse(input)?;

    let mirror_value_sum: usize = floors.iter().map(|floor| floor.mirror_value(false)).sum();
    println!("Note summarization value: {mirror_value_sum}");

    let smudged_mirror_value_sum: usize = floors.iter().map(|floor| floor.mirror_value(true)).sum();
    println!("Smudged note summarization value: {smudged_mirror_value_sum}");
    Ok(())
}
