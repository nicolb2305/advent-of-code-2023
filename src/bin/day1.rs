use color_eyre::{eyre::ContextCompat, Result};
use parse::parse;

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        combinator::map,
        multi::many1,
        IResult,
    };

    fn number_from_string(i: &str) -> IResult<&str, u32> {
        let parse_num =
            |num_str, num_name, num_int| map(alt((tag(num_str), tag(num_name))), move |_| num_int);
        let (_, num) = alt((
            parse_num("1", "one", 1),
            parse_num("2", "two", 2),
            parse_num("3", "three", 3),
            parse_num("4", "four", 4),
            parse_num("5", "five", 5),
            parse_num("6", "six", 6),
            parse_num("7", "seven", 7),
            parse_num("8", "eight", 8),
            parse_num("9", "nine", 9),
        ))(i)?;

        let (i, _) = take(1usize)(i)?;
        Ok((i, num))
    }

    fn any(i: &str) -> IResult<&str, Option<u32>> {
        alt((map(number_from_string, Some), map(take(1usize), |_| None)))(i)
    }

    pub fn parse(i: &str) -> Option<Vec<u32>> {
        let (_, nums) = many1(any)(i).ok()?;
        Some(nums.into_iter().flatten().collect())
    }
}

fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| {
            let first = l.chars().find_map(|c| c.to_digit(10))?;
            let last = l.chars().rev().find_map(|c| c.to_digit(10))?;
            Some(first * 10 + last)
        })
        .sum()
}

fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(parse)
        .map(|res| {
            res.and_then(|list| {
                let first = list.first()?;
                let last = list.last()?;
                Some(first * 10 + last)
            })
        })
        .sum()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day1.txt");

    let calibration_value1 = part_one(input).wrap_err("no digits found")?;
    println!("First calibration value: {calibration_value1}");

    let calibration_value2 = part_two(input).wrap_err("no digits found")?;
    println!("Second calibration value: {calibration_value2}");
    Ok(())
}
