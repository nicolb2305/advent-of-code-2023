use anyhow::Result;
use num::Integer;
use parse::parse;
use std::collections::HashMap;

#[derive(Debug)]
struct Graph<T>(HashMap<T, Node<T>>);

impl<'a> Graph<&'a str> {
    fn count_steps(&self, directions: &[Direction]) -> Option<usize> {
        let mut current_node = self.0.get("AAA")?;
        for (count, dir) in directions.iter().cycle().enumerate() {
            let next_node = *current_node.turn(*dir);
            if next_node == "ZZZ" {
                return Some(count + 1);
            }
            current_node = self.0.get(next_node)?;
        }
        None
    }

    fn count_steps_multiple(&self, directions: &[Direction]) -> Option<usize> {
        self.0
            .keys()
            .filter(|name| name.as_bytes()[2] == b'A')
            .copied()
            .map(|name| {
                let mut current_node = self.0.get(name).unwrap();
                let mut next_node_name = name;
                let mut visited = vec![];
                for dir in directions.iter().cycle() {
                    next_node_name = current_node.turn(*dir);
                    if visited.contains(&next_node_name) {
                        break;
                    }
                    visited.push(next_node_name);
                    current_node = self.0.get(next_node_name).unwrap();
                }
                let loop_start = visited
                    .iter()
                    .position(|&name| name == next_node_name)
                    .unwrap();
                let loop_length = visited.len() - loop_start;
                loop_length.lcm(&directions.len())
            })
            .reduce(|x, y| x.lcm(&y))
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node<T> {
    left: T,
    right: T,
}

impl<T> Node<T> {
    fn turn(&self, direction: Direction) -> &T {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

mod parse {
    use crate::{Direction, Graph, Node};
    use anyhow::{anyhow, Result};
    use winnow::{
        ascii::{alphanumeric1, line_ending, multispace1},
        combinator::{alt, delimited, repeat, separated, separated_pair},
        prelude::*,
    };

    fn directions(input: &mut &str) -> PResult<Vec<Direction>> {
        repeat(
            1..,
            alt(('L'.value(Direction::Left), 'R'.value(Direction::Right))),
        )
        .parse_next(input)
    }

    fn node<'a>(input: &mut &'a str) -> PResult<(&'a str, Node<&'a str>)> {
        let (name, _, (left, right)) = (
            alphanumeric1,
            " = ",
            delimited('(', separated_pair(alphanumeric1, ", ", alphanumeric1), ')'),
        )
            .parse_next(input)?;

        Ok((name, Node { left, right }))
    }

    fn parser<'a>(input: &mut &'a str) -> PResult<(Vec<Direction>, Graph<&'a str>)> {
        let (directions, _, nodes) =
            (directions, multispace1, separated(1.., node, line_ending)).parse_next(input)?;
        Ok((directions, Graph(nodes)))
    }

    pub fn parse(input: &str) -> Result<(Vec<Direction>, Graph<&str>)> {
        parser.parse(input).map_err(|e| anyhow!(e.to_string()))
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day8.txt");
    let (directions, graph) = parse(input)?;

    println!(
        "Number of steps from \"AAA\" to \"ZZZ\": {}",
        graph.count_steps(&directions).unwrap()
    );
    println!(
        "Fewest steps for each path to reach their destination at once: {}",
        graph.count_steps_multiple(&directions).unwrap()
    );
    Ok(())
}
