use color_eyre::{eyre::eyre, Result};
use derive_more::From;
use winnow::{
    ascii::{dec_uint, line_ending, multispace1},
    combinator::separated,
    prelude::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, From)]
#[repr(transparent)]
struct Milliseconds(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, From)]
#[repr(transparent)]
struct Millimeters(u64);

#[derive(Debug)]
struct Race {
    time: Milliseconds,
    distance: Millimeters,
}

impl Race {
    fn distance_travelled(&self, button_held: Milliseconds) -> Millimeters {
        let Some(remaining_time) = self.time.0.checked_sub(button_held.0) else {
            return Millimeters(0);
        };
        Millimeters(button_held.0 * remaining_time)
    }

    fn win(&self, button_held: Milliseconds) -> bool {
        self.distance_travelled(button_held) > self.distance
    }

    fn winning_moves_count(&self) -> u64 {
        let Some(first) = (1..self.time.0).find(|&button_held| self.win(Milliseconds(button_held)))
        else {
            return 0;
        };
        let Some(last) = (1..self.time.0)
            .rev()
            .find(|&button_held| self.win(Milliseconds(button_held)))
        else {
            return 0;
        };
        last - first + 1
    }
}

#[derive(Debug)]
struct Races(Vec<Race>);

impl Races {
    fn combine_races(&self) -> Result<Race> {
        let distance = Millimeters(
            self.0
                .iter()
                .map(|race| race.distance.0.to_string())
                .collect::<String>()
                .parse()?,
        );
        let time = Milliseconds(
            self.0
                .iter()
                .map(|race| race.time.0.to_string())
                .collect::<String>()
                .parse()?,
        );

        Ok(Race { time, distance })
    }
}

fn parse_nums<T>(i: &mut &str) -> PResult<Vec<T>>
where
    T: From<u64>,
{
    separated(1.., dec_uint.map(T::from), multispace1).parse_next(i)
}

fn parser(i: &mut &str) -> PResult<Races> {
    let (_, _, times, _, _, _, distances) = (
        "Time:",
        multispace1,
        parse_nums,
        line_ending,
        "Distance:",
        multispace1,
        parse_nums,
    )
        .parse_next(i)?;
    Ok(Races(
        times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect(),
    ))
}

fn parse(i: &str) -> Result<Races> {
    parser.parse(i).map_err(|e| eyre!(e.to_string()))
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day6.txt");
    let races = parse(input)?;

    println!(
        "Ways of winning multiplied: {}",
        races
            .0
            .iter()
            .map(Race::winning_moves_count)
            .product::<u64>()
    );

    println!(
        "Ways of winning large race: {}",
        races.combine_races()?.winning_moves_count()
    );

    Ok(())
}
