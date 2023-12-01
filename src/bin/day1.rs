use color_eyre::{eyre::ContextCompat, Result};
use parse::{from_digit, from_string_and_digit, parse, Parse};

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        combinator::map,
        error::{Error, ErrorKind, ParseError},
        multi::many1,
        Err, IResult, Parser,
    };

    pub trait Parse<'a, O>: Parser<&'a str, O, Error<&'a str>> {}
    impl<'a, O, T> Parse<'a, O> for T where T: Parser<&'a str, O, Error<&'a str>> {}

    pub fn from_string_and_digit(i: &str) -> IResult<&str, u32> {
        let num = |num_name, num_int| map(tag(num_name), move |_| num_int);

        let (_, num) = alt((
            from_digit,
            alt((
                num("one", 1),
                num("two", 2),
                num("three", 3),
                num("four", 4),
                num("five", 5),
                num("six", 6),
                num("seven", 7),
                num("eight", 8),
                num("nine", 9),
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

    pub fn parse<'a>(
        parser: impl Parse<'a, u32> + Copy,
    ) -> impl FnMut(&'a str) -> Option<Vec<u32>> {
        move |input| {
            let any =
                |parser| move |input| alt((map(parser, Some), map(take(1usize), |_| None)))(input);
            let (_, nums) = many1(any(parser))(input).ok()?;
            Some(nums.into_iter().flatten().collect())
        }
    }
}

fn calibrate<'a>(input: &'a str, parser: impl Parse<'a, u32> + Copy) -> Option<u32> {
    input
        .lines()
        .map(parse(parser))
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