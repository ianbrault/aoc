/*
** src/puzzles/aoc2022/day15.rs
*/

use super::Solution;
use crate::types::{Point, RangeInclusive};
use crate::utils;

use std::cmp;
use std::collections::HashSet;

#[derive(Debug)]
struct Sensor {
    pos: Point,
    closest_beacon: Point,
    beacon_distance: i64,
}

impl Sensor {
    fn parse_point(s: &str) -> Point {
        let (a, b) = utils::split(s, ", ").unwrap();
        let x = if a.starts_with("Sensor") {
            a[12..].parse().unwrap()
        } else {
            a[2..].parse().unwrap()
        };
        let y = b[2..].parse().unwrap();
        Point::new(x, y)
    }

    fn visible_range_of_row(&self, y: i64) -> RangeInclusive<i64> {
        let max_y = if y < self.pos.y {
            self.pos.y - self.beacon_distance
        } else {
            self.pos.y + self.beacon_distance
        };
        let y_dist = (max_y - y).abs();
        let x_min = self.pos.x - y_dist;
        let x_max = self.pos.x + y_dist;
        RangeInclusive::new(x_min, x_max)
    }
}

impl From<&str> for Sensor {
    fn from(value: &str) -> Self {
        let (sensor, beacon) = utils::split(value, ": closest beacon is at ").unwrap();
        let pos = Self::parse_point(sensor);
        let closest_beacon = Self::parse_point(beacon);
        let beacon_distance = Point::manhattan_distance(pos, closest_beacon);
        Self {
            pos,
            closest_beacon,
            beacon_distance,
        }
    }
}

fn filter_sensors_by_y_view(sensors: &[Sensor], y: i64) -> impl Iterator<Item = &Sensor> {
    sensors
        .iter()
        .filter(move |s| y >= s.pos.y - s.beacon_distance && y <= s.pos.y + s.beacon_distance)
}

fn get_visible_x_range_of_row(sensors: &[Sensor], y: i64) -> RangeInclusive<i64> {
    let mut x_min = i64::MAX;
    let mut x_max = i64::MIN;
    // grab all sensors that can view the target row
    for sensor in filter_sensors_by_y_view(sensors, y) {
        let x_range = sensor.visible_range_of_row(y);
        x_min = cmp::min(x_min, x_range.start);
        x_max = cmp::max(x_max, x_range.end);
    }
    RangeInclusive::new(x_min, x_max)
}

fn non_beacon_points_in_row(sensors: &[Sensor], beacons: &HashSet<Point>, y: i64) -> i64 {
    // from experimentation, this is a continuous row so iterate over the
    // sensors to find the furthest leftmost/rightmost reaches of the range
    let x_range = get_visible_x_range_of_row(sensors, y);
    // then remove any beacons from the set
    let beacons_in_row = beacons
        .iter()
        .filter(|b| b.y == y && b.x >= x_range.start && b.x <= x_range.end)
        .count() as i64;
    x_range.size() - beacons_in_row + 1
}

fn find_distress_beacon(sensors: &[Sensor]) -> Option<Point> {
    // check the visible range of each row and search for a single point gap
    for y in 0..=4000000 {
        // grab all sensors that can view this row
        let row_sensors = filter_sensors_by_y_view(sensors, y).collect::<Vec<_>>();
        // there must be at least 2 sensors that can view the row in order for
        // it to contain the distress beacon
        if row_sensors.len() < 2 {
            continue;
        }
        // get the visibility ranges of the sensors across the x-axis
        let sensor_x_ranges = row_sensors
            .iter()
            .map(|s| s.visible_range_of_row(y))
            .collect::<Vec<_>>();
        // and reduce the ranges
        let sensors_x_range = RangeInclusive::reduce(sensor_x_ranges);
        // we are looking for a single point of separation between 2 ranges
        // if this is found, this is the distress beacon
        if sensors_x_range.len() == 2 && sensors_x_range[1].start == sensors_x_range[0].end + 2 {
            return Some(Point::new(sensors_x_range[0].end + 1, y));
        }
    }
    // the distress beacon was not found
    None
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let sensors = input.split('\n').map(Sensor::from).collect::<Vec<_>>();
    let beacons = sensors
        .iter()
        .map(|s| s.closest_beacon)
        .collect::<HashSet<_>>();

    // Part A: Consult the report from the sensors you just deployed. In the row where y=2000000,
    // how many positions cannot contain a beacon?
    let points = non_beacon_points_in_row(&sensors, &beacons, 2000000);
    solution.set_part_a(points);

    // Part B: Find the only possible position for the distress beacon. What is its tuning frequency?
    if let Some(distress_beacon) = find_distress_beacon(&sensors) {
        let tuning_frequency = (distress_beacon.x * 4000000) + distress_beacon.y;
        solution.set_part_b(tuning_frequency);
    }

    solution
}
