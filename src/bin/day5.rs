use crate::parse::parse;
use std::ops::Range;

mod parse {
    use crate::{Almanac, Map, Mapping};
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, line_ending, multispace1, u64 as nom_u64},
        multi::{count, separated_list1},
        sequence::tuple,
        IResult,
    };

    fn mapping(i: &str) -> IResult<&str, Mapping> {
        let (i, (dest_start, _, src_start, _, length)) =
            tuple((nom_u64, multispace1, nom_u64, multispace1, nom_u64))(i)?;
        Ok((
            i,
            Mapping {
                source: src_start..(src_start + length),
                destination: dest_start..(dest_start + length),
            },
        ))
    }

    fn map(i: &str) -> IResult<&str, Map> {
        let (i, (_, _, _, _, _, mappings)) = tuple((
            alpha1,
            tag("-to-"),
            alpha1,
            tag(" map:"),
            multispace1,
            separated_list1(line_ending, mapping),
        ))(i)?;
        Ok((i, Map { mappings }))
    }

    pub fn parse(i: &str) -> IResult<&str, Almanac> {
        let (i, (_, _, seeds, _, maps)) = tuple((
            tag("seeds:"),
            multispace1,
            separated_list1(multispace1, nom_u64),
            multispace1,
            separated_list1(count(line_ending, 2), map),
        ))(i)?;
        let seed_ranges = seeds.chunks(2).map(|x| x[0]..(x[0] + x[1])).collect();
        Ok((
            i,
            Almanac {
                seeds,
                seed_ranges,
                maps,
            },
        ))
    }
}

#[derive(Debug)]
struct Mapping {
    source: Range<u64>,
    destination: Range<u64>,
}

impl Mapping {
    fn map(&self, input: u64) -> Option<u64> {
        if self.source.contains(&input) {
            Some(self.destination.start + (input - self.source.start))
        } else {
            None
        }
    }

    // Return a tuple of mapped-to range and vector of ranges that were not mapped
    #[allow(clippy::single_range_in_vec_init)]
    fn map_range(&self, input: &Range<u64>) -> (Option<Range<u64>>, Option<Vec<Range<u64>>>) {
        match (self.map(input.start), self.map(input.end)) {
            // All locations are mapped, nothing unmapped
            (Some(start), Some(end)) => (Some(start..end), None),
            // Start of input is mapped, remainder is not
            (Some(start), None) => (
                Some(start..self.destination.end),
                Some(vec![self.source.end..input.end]),
            ),
            // End of input is mapped, start is not
            (None, Some(end)) => (
                Some(self.destination.start..end),
                Some(vec![input.start..self.source.start]),
            ),
            // Unknown amount is mapped
            (None, None) => {
                if input.end <= self.source.start || input.start >= self.source.end {
                    // Range is entirely before or after mapping range, nothing is mapped
                    (None, Some(vec![input.clone()]))
                } else {
                    // Some part in the middle of input range is mapped, start and end are not
                    (
                        Some(self.destination.clone()),
                        Some(vec![
                            input.start..self.source.start,
                            self.source.end..input.end,
                        ]),
                    )
                }
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn map(&self, input: u64) -> u64 {
        match self.mappings.iter().find_map(|map| map.map(input)) {
            Some(v) => v,
            None => input,
        }
    }

    fn map_range(&self, input: &[Range<u64>]) -> Vec<Range<u64>> {
        // Locations that have been mapped to
        let mut all_mapped = vec![];
        // Map all input ranges and unmapped ranges from previous maps
        let all_unmapped = self.mappings.iter().fold(input.to_vec(), |v, map| {
            let mut all_unmapped = vec![];
            for (mapped, unmapped) in v.iter().map(|range| map.map_range(range)) {
                if let Some(mapped) = mapped {
                    all_mapped.push(mapped);
                }
                if let Some(unmapped) = unmapped {
                    for unmapped in unmapped {
                        all_unmapped.push(unmapped);
                    }
                }
            }
            all_unmapped
        });
        // All remaning ranges unmapped ranges map to themselves
        all_mapped
            .into_iter()
            .chain(all_unmapped)
            .filter(|range| !range.is_empty())
            .collect()
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_ranges: Vec<Range<u64>>,
    maps: Vec<Map>,
}

impl Almanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        self.maps.iter().fold(seed, |x, map| map.map(x))
    }

    fn lowest_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|&seed| self.seed_to_location(seed))
            .min()
            .unwrap()
    }

    fn seed_range_to_lowest_location(&self, seed_range: &Range<u64>) -> Vec<Range<u64>> {
        self.maps
            .iter()
            .fold(vec![seed_range.clone()], |x, map| map.map_range(&x))
    }

    fn lowest_location_ranges(&self) -> u64 {
        self.seed_ranges
            .iter()
            .flat_map(|seed_range| self.seed_range_to_lowest_location(seed_range))
            .map(|range| range.start)
            .min()
            .unwrap()
    }
}

fn main() {
    let input = include_str!("../../input/day5.txt");
    let almanac = parse(input).unwrap().1;

    println!(
        "Lowest location for individual seeds: {}",
        almanac.lowest_location()
    );
    println!(
        "Lowest location for ranges of seeds: {}",
        almanac.lowest_location_ranges()
    );
}
