use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace1, u64 as nom_u64},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
struct Milliseconds(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
        let last = (1..self.time.0)
            .rev()
            .find(|&button_held| self.win(Milliseconds(button_held)))
            .unwrap();
        last - first + 1
    }
}

#[derive(Debug)]
struct Races(Vec<Race>);

impl Races {
    fn combine_races(&self) -> Race {
        let distance = Millimeters(
            self.0
                .iter()
                .map(|race| race.distance.0.to_string())
                .collect::<String>()
                .parse()
                .unwrap(),
        );
        let time = Milliseconds(
            self.0
                .iter()
                .map(|race| race.time.0.to_string())
                .collect::<String>()
                .parse()
                .unwrap(),
        );

        Race { time, distance }
    }
}

fn parse(i: &str) -> IResult<&str, Races> {
    let (i, (_, _, times, _, _, _, distances)) = tuple((
        tag("Time:"),
        multispace1,
        separated_list1(multispace1, map(nom_u64, Milliseconds)),
        line_ending,
        tag("Distance:"),
        multispace1,
        separated_list1(multispace1, map(nom_u64, Millimeters)),
    ))(i)?;
    Ok((
        i,
        Races(
            times
                .into_iter()
                .zip(distances)
                .map(|(time, distance)| Race { time, distance })
                .collect(),
        ),
    ))
}

fn main() {
    let input = include_str!("../../input/day6.txt");
    let races = parse(input).unwrap().1;

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
        races.combine_races().winning_moves_count()
    );
}
