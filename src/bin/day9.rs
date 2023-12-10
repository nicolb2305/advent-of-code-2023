use anyhow::{anyhow, Result};
use itertools::Itertools;
use winnow::{ascii::dec_int, combinator::separated, prelude::*};

fn parser(input: &mut &str) -> PResult<Vec<i64>> {
    separated(1.., dec_int::<_, i64, _>, ' ').parse_next(input)
}

fn parse(input: &str) -> Result<Vec<i64>> {
    parser.parse(input).map_err(|e| anyhow!(e.to_string()))
}

fn row_difference(row: impl Iterator<Item = i64> + Clone) -> i64 {
    let mut final_element = vec![];
    let mut difference: Vec<_> = row.collect();
    while difference.iter().any(|&x| x != 0) {
        final_element.push(difference[difference.len() - 1]);
        difference = difference
            .iter()
            .tuple_windows()
            .map(|(x, y)| y - x)
            .collect();
    }
    final_element.into_iter().sum()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day9.txt");
    let sensor_data = input.lines().map(parse).collect::<Result<Vec<_>>>()?;

    let prediction: i64 = sensor_data
        .iter()
        .map(|row| row_difference(row.iter().copied()))
        .sum();
    println!("Sum of predicted next value: {prediction}");

    let history: i64 = sensor_data
        .iter()
        .map(|row| row_difference(row.iter().rev().copied()))
        .sum();
    println!("Sum of previous historical value: {history}");

    Ok(())
}
