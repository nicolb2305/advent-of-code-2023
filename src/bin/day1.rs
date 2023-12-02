use color_eyre::{eyre::ContextCompat, Result};
use parse::{from_digit, from_string_and_digit, parse, Parse};

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        combinator::{map, value},
        error::{Error, ErrorKind, ParseError},
        multi::many1,
        Err, IResult, Parser,
    };

    pub trait Parse<'a>: Parser<&'a str, u32, Error<&'a str>> + Copy {}
    impl<'a, T> Parse<'a> for T where T: Parser<&'a str, u32, Error<&'a str>> + Copy {}

    pub fn from_string_and_digit(i: &str) -> IResult<&str, u32> {
        let (_, num) = alt((
            from_digit,
            alt((
                value(1, tag("one")),
                value(2, tag("two")),
                value(3, tag("three")),
                value(4, tag("four")),
                value(5, tag("five")),
                value(6, tag("six")),
                value(7, tag("seven")),
                value(8, tag("eight")),
                value(9, tag("nine")),
            )),
        ))(i)?;

        let (i, _) = take(1usize)(i)?;
        Ok((i, num))
    }

    pub fn from_digit(i: &str) -> IResult<&str, u32> {
        let (i, num) = take(1usize)(i)?;
        num.parse()
            .map(|v| (i, v))
            .map_err(|_| Err::Error(Error::from_error_kind(i, ErrorKind::IsNot)))
    }

    pub fn parse<'a>(input: &'a str, parser: impl Parse<'a>) -> Option<Vec<u32>> {
        let (_, nums) = many1(alt((map(parser, Some), map(take(1usize), |_| None))))(input).ok()?;
        Some(nums.into_iter().flatten().collect())
    }
}

fn calibrate<'a>(input: &'a str, parser: impl Parse<'a>) -> Option<u32> {
    input
        .lines()
        .map(|input| parse(input, parser))
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

    let calibration_value1 = calibrate(input, from_digit).wrap_err("no digits found")?;
    println!("First calibration value: {calibration_value1}");

    let calibration_value2 = calibrate(input, from_string_and_digit).wrap_err("no digits found")?;
    println!("Second calibration value: {calibration_value2}");
    Ok(())
}
