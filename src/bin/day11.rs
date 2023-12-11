use advent_of_code_2023::coordinates::Coordinate;
use anyhow::{anyhow, Result};
use itertools::Itertools;

struct Galaxies {
    galaxies: Vec<Coordinate>,
    empty_columns: Vec<usize>,
    empty_rows: Vec<usize>,
}

impl Galaxies {
    fn num_empty_cols_rows_between_coords(&self, from: Coordinate, to: Coordinate) -> usize {
        (from.x.min(to.x)..=from.x.max(to.x))
            .filter(|i| self.empty_columns.contains(i))
            .count()
            + (from.y.min(to.y)..=from.y.max(to.y))
                .filter(|j| self.empty_rows.contains(j))
                .count()
    }

    fn sum_of_pair_distances(&self, multiplier: usize) -> usize {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(x, y)| {
                x.manhatten_distance(*y)
                    + self.num_empty_cols_rows_between_coords(*x, *y) * (multiplier - 1)
            })
            .sum()
    }
}

fn parse(input: &str) -> Option<Galaxies> {
    let galaxies: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, char)| {
                if char == '#' {
                    Some(Coordinate::new(j, i))
                } else {
                    None
                }
            })
        })
        .collect();

    let width = galaxies.iter().map(|coord| coord.x).max()? + 1;
    let height = galaxies.iter().map(|coord| coord.y).max()? + 1;

    let empty_columns: Vec<_> = (0..width)
        .filter(|x| !galaxies.iter().any(|coord| coord.x == *x))
        .collect();

    let empty_rows: Vec<_> = (0..height)
        .filter(|y| !galaxies.iter().any(|coord| coord.y == *y))
        .collect();

    Some(Galaxies {
        galaxies,
        empty_columns,
        empty_rows,
    })
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day11.txt");
    let galaxies = parse(input).ok_or(anyhow!("failed to parse"))?;

    println!(
        "Distance between galaxies with 2x expansion: {}",
        galaxies.sum_of_pair_distances(2)
    );
    println!(
        "Distance between galaxies with 1,000,000x expansion: {}",
        galaxies.sum_of_pair_distances(1_000_000)
    );

    Ok(())
}
