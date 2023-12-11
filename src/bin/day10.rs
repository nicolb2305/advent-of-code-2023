use advent_of_code_2023::coordinates::{Coordinate, Grid, Offset};
use anyhow::{anyhow, Error, Result};
use std::fmt::Display;
use winnow::{
    ascii::line_ending,
    combinator::{alt, repeat, separated},
    prelude::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

impl From<Direction> for Offset {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Offset::new(0, -1),
            Direction::East => Offset::new(1, 0),
            Direction::South => Offset::new(0, 1),
            Direction::West => Offset::new(-1, 0),
        }
    }
}

impl TryFrom<Offset> for Direction {
    type Error = Error;

    fn try_from(value: Offset) -> Result<Self> {
        Ok(match value {
            Offset { dx: 0, dy: -1 } => Direction::North,
            Offset { dx: 1, dy: 0 } => Direction::East,
            Offset { dx: 0, dy: 1 } => Direction::South,
            Offset { dx: -1, dy: 0 } => Direction::West,
            _ => return Err(anyhow!("cannot convert {value:?} to direction")),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Empty,
    Path(Direction, Direction),
    Start,
}

impl Node {
    fn walk(self, direction: Direction) -> Option<Direction> {
        match self {
            Node::Empty | Node::Start => None,
            Node::Path(x, y) => {
                if x.opposite() == direction {
                    Some(y)
                } else if y.opposite() == direction {
                    Some(x)
                } else {
                    None
                }
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => ' ',
                Self::Start => 'S',
                Self::Path(Direction::West, Direction::East)
                | Self::Path(Direction::East, Direction::West) => '\u{2550}',
                Self::Path(Direction::North, Direction::South)
                | Self::Path(Direction::South, Direction::North) => '\u{2551}',
                Self::Path(Direction::South, Direction::East)
                | Self::Path(Direction::East, Direction::South) => '\u{2554}',
                Self::Path(Direction::West, Direction::South)
                | Self::Path(Direction::South, Direction::West) => '\u{2557}',
                Self::Path(Direction::North, Direction::East)
                | Self::Path(Direction::East, Direction::North) => '\u{255A}',
                Self::Path(Direction::North, Direction::West)
                | Self::Path(Direction::West, Direction::North) => '\u{255D}',
                Self::Path(_, _) => unimplemented!(),
            }
        )
    }
}

#[derive(Debug)]
struct Maze(Grid<Node>);

impl Maze {
    fn start_point(&self) -> Option<Coordinate> {
        self.0.find(&Node::Start)
    }

    fn find_length(&self) -> Option<(Vec<Coordinate>, Vec<Direction>)> {
        let start = self.start_point()?;
        let mut current = start.iter(false).find(|coord| match self.0.get(*coord) {
            Some(Node::Empty | Node::Start) | None => false,
            Some(Node::Path(x, y)) => {
                Offset::from(*x) == (start - *coord) || Offset::from(*y) == (start - *coord)
            }
        })?;

        let mut current_node = self.0.get(current)?;
        let mut prev_direction = (current - start).try_into().ok()?;
        Some(
            [(current, prev_direction)]
                .into_iter()
                .chain(std::iter::from_fn(move || {
                    prev_direction = current_node.walk(prev_direction)?;
                    current = current.offset(prev_direction.into())?;
                    current_node = self.0.get(current)?;
                    Some((current, prev_direction))
                }))
                .unzip(),
        )
    }

    fn count_enclosed(&self, coordinates: &[Coordinate], directions: &[Direction]) -> usize {
        let mut i = 0;
        for (y, row) in self.0 .0.iter().enumerate() {
            let mut num = 0;
            for (x, _) in row.iter().enumerate() {
                if let Some(index) = coordinates
                    .iter()
                    .position(|coord| *coord == Coordinate::new(x, y))
                {
                    let dir = directions[index];
                    let node = self.0.get(coordinates[index]).unwrap();
                    match node {
                        Node::Path(_, Direction::South) | Node::Path(Direction::South, _) => {
                            if dir == Direction::North {
                                num += 1;
                            } else {
                                num -= 1;
                            }
                        }
                        _ => {}
                    }
                } else if num != 0 {
                    i += 1;
                }
            }
        }
        i
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out_string = self
            .0
             .0
            .iter()
            .map(|row| row.iter().map(ToString::to_string).collect())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{out_string}")
    }
}

fn parse_nodes(input: &mut &str) -> PResult<Vec<Node>> {
    repeat(
        1..,
        alt((
            '|'.value(Node::Path(Direction::North, Direction::South)),
            '-'.value(Node::Path(Direction::West, Direction::East)),
            'L'.value(Node::Path(Direction::North, Direction::East)),
            'J'.value(Node::Path(Direction::North, Direction::West)),
            '7'.value(Node::Path(Direction::South, Direction::West)),
            'F'.value(Node::Path(Direction::South, Direction::East)),
            '.'.value(Node::Empty),
            'S'.value(Node::Start),
        )),
    )
    .parse_next(input)
}

fn parser(input: &mut &str) -> PResult<Maze> {
    separated(1.., parse_nodes, line_ending)
        .map(|maze| Maze(Grid(maze)))
        .parse_next(input)
}

fn parse(input: &str) -> Result<Maze> {
    parser.parse(input).map_err(|e| anyhow!(e.to_string()))
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day10.txt");
    let maze = parse(input)?;
    println!("{maze}");

    let (path, directions) = maze.find_length().unwrap();
    println!("Furthest distance from start: {}", path.len() / 2);

    println!(
        "Enclosed spaces: {}",
        maze.count_enclosed(&path, &directions)
    );
    Ok(())
}
