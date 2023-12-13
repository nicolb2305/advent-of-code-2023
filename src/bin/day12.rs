use anyhow::{anyhow, Result};
use memoize::memoize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fmt::{Display, Write};
use winnow::{
    ascii::{dec_uint, multispace1},
    combinator::{alt, repeat, separated, separated_pair},
    prelude::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Operational => '.',
                Self::Damaged => '#',
                Self::Unknown => '?',
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Springs(Vec<Spring>);

impl Display for Springs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().fold(String::new(), |mut out, spring| {
                let _ = write!(out, "{spring}");
                out
            })
        )
    }
}

#[derive(Debug, Clone)]
struct Row {
    springs: Springs,
    spring_groups: Vec<u64>,
}

fn parse_springs(input: &mut &str) -> PResult<Springs> {
    repeat(
        1..,
        alt((
            '.'.value(Spring::Operational),
            '#'.value(Spring::Damaged),
            '?'.value(Spring::Unknown),
        )),
    )
    .map(Springs)
    .parse_next(input)
}

fn parser(input: &mut &str) -> PResult<Row> {
    let (springs, spring_groups) = separated_pair(
        parse_springs,
        multispace1,
        separated(1.., dec_uint::<_, u64, _>, ','),
    )
    .parse_next(input)?;
    Ok(Row {
        springs,
        spring_groups,
    })
}

fn parse(input: &str) -> Result<Row> {
    parser.parse(input).map_err(|e| anyhow!(e.to_string()))
}

#[memoize]
fn recursive_check(
    remaining: &'static [Spring],
    to_place: &'static [u64],
    num_placed: u64,
    currently_in_a_row: u64,
    current_group: usize,
) -> u64 {
    match to_place.get(current_group) {
        None if currently_in_a_row > 0 => return 0,
        Some(v) if *v < currently_in_a_row => return 0,
        _ => {}
    }

    let operational = || {
        recursive_check(
            &remaining[1..],
            to_place,
            num_placed,
            0,
            current_group + usize::from(currently_in_a_row > 0),
        )
    };
    let damaged = || {
        recursive_check(
            &remaining[1..],
            to_place,
            num_placed + 1,
            currently_in_a_row + 1,
            current_group,
        )
    };

    match remaining.first() {
        None => (num_placed == to_place.iter().sum()).into(),
        Some(Spring::Operational) => operational(),
        Some(Spring::Damaged) => damaged(),
        Some(Spring::Unknown) => damaged() + operational(),
    }
}

fn repeat_and_leak(row: &Row, amount: usize) -> (&'static [Spring], &'static [u64]) {
    let springs = std::iter::repeat(row.springs.0.iter().copied().chain([Spring::Unknown]))
        .take(amount)
        .flatten()
        .take(row.springs.0.len() * amount + (amount - 1))
        .collect::<Vec<_>>()
        .leak();
    let spring_grouos = std::iter::repeat(row.spring_groups.iter().copied())
        .take(amount)
        .flatten()
        .collect::<Vec<_>>()
        .leak();
    (springs, spring_grouos)
}

fn num_arrangements(rows: &[Row], amount: usize) -> u64 {
    rows.par_iter()
        .map(|row| {
            let (springs, spring_groups) = repeat_and_leak(row, amount);
            recursive_check(springs, spring_groups, 0, 0, 0)
        })
        .sum()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day12.txt");
    let rows = input.lines().map(parse).collect::<Result<Vec<_>>>()?;

    let arrangements = num_arrangements(&rows, 1);
    println!("Number of possible arrangements: {arrangements}");

    let arrangements_x5 = num_arrangements(&rows, 5);
    println!("Number of possible arrangements for input of 5x size: {arrangements_x5}");
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{parse, recursive_check, repeat_and_leak, Row};
    use anyhow::Result;

    fn read_data() -> Result<Vec<Row>> {
        let input = include_str!("../../input/day12test.txt");
        input.lines().map(parse).collect()
    }

    #[test]
    fn recursive() -> Result<()> {
        let rows = read_data()?;

        let (springs, spring_groups) = repeat_and_leak(&rows[0], 1);
        assert_eq!(recursive_check(springs, spring_groups, 0, 0, 0), 1);

        let (springs, spring_groups) = repeat_and_leak(&rows[1], 1);
        assert_eq!(recursive_check(springs, spring_groups, 0, 0, 0), 4);

        let (springs, spring_groups) = repeat_and_leak(&rows[2], 1);
        assert_eq!(recursive_check(springs, spring_groups, 0, 0, 0), 1);

        let (springs, spring_groups) = repeat_and_leak(&rows[3], 1);
        assert_eq!(recursive_check(springs, spring_groups, 0, 0, 0), 1);

        let (springs, spring_groups) = repeat_and_leak(&rows[4], 1);
        assert_eq!(recursive_check(springs, spring_groups, 0, 0, 0), 4);

        let (springs, spring_groups) = repeat_and_leak(&rows[5], 1);
        assert_eq!(recursive_check(springs, spring_groups, 0, 0, 0), 10);
        Ok(())
    }

    #[test]
    fn recursive2() -> Result<()> {
        let rows = read_data()?;

        let (springs, spring_groups) = repeat_and_leak(&rows[0], 5);
        assert_eq!(recursive_check(springs, spring_groups, 0, 0, 0), 1);

        let (springs, spring_groups) = repeat_and_leak(&rows[1], 5);
        assert_eq!(recursive_check(springs, spring_groups, 0, 0, 0), 16384);

        let (springs, spring_groups) = repeat_and_leak(&rows[2], 5);
        assert_eq!(recursive_check(springs, spring_groups, 0, 0, 0), 1);

        let (springs, spring_groups) = repeat_and_leak(&rows[3], 5);
        assert_eq!(recursive_check(springs, spring_groups, 0, 0, 0), 16);

        let (springs, spring_groups) = repeat_and_leak(&rows[4], 5);
        assert_eq!(recursive_check(springs, spring_groups, 0, 0, 0), 2500);

        let (springs, spring_groups) = repeat_and_leak(&rows[5], 5);
        assert_eq!(recursive_check(springs, spring_groups, 0, 0, 0), 506_250);
        Ok(())
    }
}
