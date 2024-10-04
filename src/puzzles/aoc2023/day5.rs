/*
** src/puzzles/aoc2023/day5.rs
*/

use super::Solution;
use crate::itertools::*;
use crate::utils;

use case_iterable::CaseIterable;
use log::debug;

use std::cmp;
use std::collections::VecDeque;

#[derive(CaseIterable, Clone, Copy, Debug)]
enum Resource {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Clone)]
struct RangeMap {
    source_start: i64,
    source_end: i64,
    offset: i64,
}

impl RangeMap {
    fn get(&self, from: i64) -> i64 {
        from + self.offset
    }
}

impl From<&str> for RangeMap {
    fn from(value: &str) -> Self {
        let parts = utils::split_and_parse(value).collect::<Vec<_>>();
        match parts.as_slice() {
            &[destination_start, source_start, length] => Self {
                source_start,
                source_end: source_start + length,
                offset: destination_start - source_start,
            },
            _ => panic!("Range::From<&str>: invalid string: {}", value),
        }
    }
}

struct RangeMapList {
    list: Vec<RangeMap>,
    source_min: i64,
}

impl RangeMapList {
    fn get(&self, source: i64) -> i64 {
        if source < self.source_min {
            return source;
        }
        for range in self.list.iter() {
            if source <= range.source_end {
                return range.get(source);
            }
        }
        source
    }
}

impl From<Vec<RangeMap>> for RangeMapList {
    fn from(value: Vec<RangeMap>) -> Self {
        let mut list = value.clone();
        list.sort_by_key(|range| range.source_start);
        let source_min = list.first().unwrap().source_start;

        Self { list, source_min }
    }
}

struct Almanac {
    seeds: Vec<i64>,
    transformers: Vec<RangeMapList>,
}

impl Almanac {
    fn seed_pairs(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        self.seeds.iter().paired().map(|(&a, &b)| (a, a + b))
    }

    fn location(&self, seed: i64) -> i64 {
        let mut output = seed;
        for resource in Resource::all_cases() {
            if matches!(resource, Resource::Location) {
                break;
            }
            debug!("{:?}: {}", resource, output);
            output = self.transformers[resource as usize].get(output);
        }
        debug!("{:?}: {}", Resource::Location, output);
        output
    }

    fn parse_range_list(input: &str) -> RangeMapList {
        input
            .split('\n')
            .skip(1)
            .map(RangeMap::from)
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<String> for Almanac {
    fn from(value: String) -> Self {
        let chunks = value.split("\n\n").collect::<Vec<_>>();
        let seeds = utils::split_and_parse(chunks[0]).collect();
        let transformers = chunks[1..]
            .iter()
            .map(|chunk| Self::parse_range_list(chunk))
            .collect();
        Self {
            seeds,
            transformers,
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The almanac (your puzzle input) lists all of the seeds that need to be planted. It also
    // lists what type of soil to use with each kind of seed, what type of fertilizer to use with
    // each kind of soil, what type of water to use with each kind of fertilizer, and so on.
    let almanac = Almanac::from(input);

    // Part A: What is the lowest location number that corresponds to any of the initial
    // seed numbers?
    let lowest_location = almanac
        .seeds
        .iter()
        .map(|&seed| almanac.location(seed))
        .min();
    solution.maybe_set_part_a(lowest_location);

    // Part B: Re-reading the almanac, it looks like the seeds: line actually describes ranges of
    // seed numbers. Consider all of the initial seed numbers listed in the ranges on the first
    // line of the almanac. What is the lowest location number that corresponds to any of the
    // initial seed numbers?
    let mut lowest_location_paired = i64::MAX;
    // process each seed pair as an interval, converting between resources individually this time
    let mut work_queue = almanac
        .seed_pairs()
        .map(|(start, end)| (start, end, Resource::Seed))
        .collect::<VecDeque<_>>();
    while let Some((mut start, mut end, resource)) = work_queue.pop_front() {
        if matches!(resource, Resource::Location) {
            lowest_location_paired = cmp::min(lowest_location_paired, start);
        } else {
            let mut overlap = false;
            for transformer in almanac.transformers[resource as usize].list.iter() {
                // for each resource transformer, consider the overlap of the input source range
                // with the source range for the transformer:
                // (a) no overlap: continue on
                if end <= transformer.source_start || start >= transformer.source_end {
                    continue;
                }
                // (b) partial overlap: split at the intersection, add the non-overlapping portion
                // to the work queue and continue on with the overlapping portion
                if start < transformer.source_start {
                    work_queue.push_back((start, transformer.source_start, resource));
                    start = transformer.source_start;
                }
                if transformer.source_end < end {
                    work_queue.push_back((transformer.source_end, end, resource));
                    end = transformer.source_end;
                }
                // convert the overlapping range to the next resource
                work_queue.push_back((
                    start + transformer.offset,
                    end + transformer.offset,
                    resource.next().unwrap(),
                ));
                overlap = true;
            }
            if !overlap {
                // no overlap found for the range: continue to the next resource
                work_queue.push_back((start, end, resource.next().unwrap()));
            }
        }
    }
    solution.set_part_b(lowest_location_paired);

    solution
}
