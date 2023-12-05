use crate::parse::parse;
use rayon::prelude::*;
use std::ops::Range;

trait SetOperations
where
    Self: Sized,
{
    fn disjoint(&self, other: &Self) -> bool;
    fn contained_in(&self, other: &Self) -> bool;
    fn overlap(&self, other: &Self) -> bool;
    fn adjacent(&self, other: &Self) -> bool;
    fn union(&self, other: &Self) -> Option<Self>;
    fn intersection(&self, other: &Self) -> Option<Self>;
}

impl<T> SetOperations for Range<T>
where
    T: Ord + Copy,
{
    fn disjoint(&self, other: &Self) -> bool {
        self.end <= other.start || self.start >= other.end
    }

    fn contained_in(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn overlap(&self, other: &Self) -> bool {
        (self.start >= other.start && self.start < other.end)
            || (self.end <= other.end && self.end > other.start)
    }

    fn adjacent(&self, other: &Self) -> bool {
        self.start == other.end || self.end == other.start
    }

    fn union(&self, other: &Self) -> Option<Self> {
        if self.overlap(other) || self.adjacent(other) {
            Some(self.start.min(other.start)..self.end.max(other.end))
        } else {
            None
        }
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.overlap(other) {
            Some(self.start.max(other.start)..self.end.min(other.end))
        } else {
            None
        }
    }
}

struct RangeMapping {
    mapped: Option<Range<u64>>,
    unmapped1: Option<Range<u64>>,
    unmapped2: Option<Range<u64>>,
}

impl RangeMapping {
    fn from_mapped(mapped: Range<u64>) -> Self {
        Self {
            mapped: Some(mapped),
            unmapped1: None,
            unmapped2: None,
        }
    }

    fn from_unmapped(unmapped: Range<u64>) -> Self {
        Self {
            mapped: None,
            unmapped1: Some(unmapped),
            unmapped2: None,
        }
    }

    fn from_both(mapped: Range<u64>, unmapped1: Range<u64>, unmapped2: Option<Range<u64>>) -> Self {
        Self {
            mapped: Some(mapped),
            unmapped1: Some(unmapped1),
            unmapped2,
        }
    }
}

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
    fn map_range(&self, input: &Range<u64>) -> RangeMapping {
        match (self.map(input.start), self.map(input.end)) {
            // All locations are mapped, nothing unmapped
            (Some(start), Some(end)) => RangeMapping::from_mapped(start..end),
            // Start of input is mapped, remainder is not
            (Some(start), None) => RangeMapping::from_both(
                start..self.destination.end,
                self.source.end..input.end,
                None,
            ),
            // End of input is mapped, start is not
            (None, Some(end)) => RangeMapping::from_both(
                self.destination.start..end,
                input.start..self.source.start,
                None,
            ),
            // Unknown amount is mapped
            (None, None) => {
                if input.end <= self.source.start || input.start >= self.source.end {
                    // Range is entirely before or after mapping range, nothing is mapped
                    RangeMapping::from_unmapped(input.clone())
                } else {
                    // Some part in the middle of input range is mapped, start and end are not
                    RangeMapping::from_both(
                        self.destination.clone(),
                        input.start..self.source.start,
                        Some(self.source.end..input.end),
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
        let mut all_mapped = Vec::with_capacity(input.len());
        // Map all input ranges and unmapped ranges from previous maps
        let all_unmapped = self.mappings.iter().fold(input.to_vec(), |v, map| {
            let mut all_unmapped = Vec::with_capacity(v.len());
            for RangeMapping {
                mapped,
                unmapped1,
                unmapped2,
            } in v.iter().map(|range| map.map_range(range))
            {
                if let Some(mapped) = mapped {
                    all_mapped.push(mapped);
                }
                if let Some(unmapped) = unmapped1 {
                    all_unmapped.push(unmapped);
                }
                if let Some(unmapped) = unmapped2 {
                    all_unmapped.push(unmapped);
                }
            }
            all_unmapped
        });
        // All remaining ranges unmapped ranges map to themselves
        let mut mapped: Vec<_> = all_mapped
            .into_iter()
            .chain(all_unmapped)
            .filter(|range| !range.is_empty())
            .collect();
        let mut prev_len = mapped.len() + 1;
        // Iteratively perform unions of ranges until all ranges are disjoint
        while mapped.len() < prev_len {
            prev_len = mapped.len();
            mapped = mapped.into_iter().fold(vec![], |mut v, range| {
                if let Some((i, union)) = v
                    .iter()
                    .enumerate()
                    .find_map(|(i, other)| range.union(other).map(move |union| (i, union)))
                {
                    v.remove(i);
                    v.push(union);
                } else {
                    v.push(range);
                }
                v
            });
        }
        mapped
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
            .par_iter()
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
    assert_eq!(almanac.lowest_location(), 388_071_289);
    println!(
        "Lowest location for ranges of seeds: {}",
        almanac.lowest_location_ranges()
    );
    assert_eq!(almanac.lowest_location_ranges(), 84_206_669);
}
