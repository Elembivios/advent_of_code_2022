use std::{ops::Range};
use std::str::FromStr;
use anyhow::{Error, Context};
use crate::utils::point::Coord;

type C = Coord<i64>;

struct Sensor {
    coord: C,
    beacon: C,
    distance: i64
}

impl FromStr for Sensor {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once(": ").context("Invalid Sensor string")?;
        let lhs: Vec<i64> = lhs.strip_prefix("Sensor at ").context("Invalid sensor string")?.split(", ").map(|part| {
            let value = part.split("=").last().context("No '=' sign to split on.")?;
            value.parse().context("Value couldn't be parsed to integer")
        }).collect::<Result<_, _>>()?;
        let rhs: Vec<i64> = rhs.strip_prefix("closest beacon is at ").context("Invalid sensor string")?.split(", ").map(|part| {
            let value = part.split("=").last().context("No '=' sign to split on.")?;
            value.parse().context("Value couldn't be parsed to integer")
        }).collect::<Result<_, _>>()?;
        let sensor = Coord::new(lhs[0], lhs[1]);
        let beacon = Coord::new(rhs[0], rhs[1]);
        Ok(Sensor::new(sensor, beacon))
    }
}

impl Sensor {
    fn new(sensor: C, beacon: C) -> Self {
        let distance = sensor.manhattan_distance(&beacon);
        Sensor { coord: sensor, beacon, distance }
    }

    fn within_sensor_range(&self, other: &C) -> bool {
        other.manhattan_distance(&self.coord) <= self.distance
    }

    fn can_contain_unseen_points(&self, min: C, max: C) -> bool {
        let corners = [
            Coord::new(min.x, min.y),
            Coord::new(min.x, max.y),
            Coord::new(max.x, min.y),
            Coord::new(max.x, max.y),
        ];
        let largest_dist = corners.into_iter().map(|corner| {
            corner.manhattan_distance(&self.coord)
        }).max().unwrap();
        largest_dist > self.distance
    }
}

pub struct BeaconExclusionZone {
    sensors: Vec<Sensor>
}

impl crate::Advent for BeaconExclusionZone {
    fn new(data: &str) -> Self
    where 
    Self: Sized 
    {
        let sensors: Vec<Sensor> = data.lines().map(|l| {
            Sensor::from_str(l).unwrap()
        }).collect();  
        BeaconExclusionZone { sensors }
    }
    fn part_01(&self) -> String {
        let y = 2_000_000;
        // let y = 10; // For example input
        let lower_limit_x = self.sensors.iter().map(|s| s.coord.x - s.distance ).min().unwrap();
        let upper_limit_x = self.sensors.iter().map(|s| s.coord.x + s.distance ).max().unwrap();
        let full_ranges = beacon_covered_at_y(&self.sensors, y, lower_limit_x, upper_limit_x);
        let mut to_exclude: Vec<i64> = self.sensors.iter().map(|s| {
            vec![s.beacon, s.coord]
        }).flatten().filter_map(|o| {
            if o.y == y {
                Some(o.x)
            } else {
                None
            }
        }).collect();
        to_exclude.sort_unstable();
        to_exclude.dedup();

        let result = full_ranges.iter().map(|range| {
            let mut len = range.end - range.start;
            for exclude in to_exclude.iter() {
                if range.contains(exclude) {
                    len -= 1;
                }
            }
            len
        }).sum::<i64>();
        result.to_string()
    }

    fn part_02(&self) -> String {
        let lower_limit = 0;
        let upper_limit = 4_000_000;
        // let upper_limit = 20; // For example input
        let min = Coord::new(lower_limit, lower_limit);
        let max = Coord::new(upper_limit, upper_limit);
        let empty_space = find_useen_point(&self.sensors, min, max).unwrap();
        let result = (empty_space.x * 4_000_000) + empty_space.y;
        result.to_string()
    }
}

fn beacon_covered_at_y(sensors: &Vec<Sensor>, y: i64, lower_limit_x: i64, upper_limit_x: i64) -> Vec<Range<i64>> {
    let mut sensors: Vec<_> = sensors.iter().filter_map(|s| {
        let y_diff = s.coord.y.abs_diff(y) as i64;
        if y_diff > s.distance {
            return None;
        }
        let x_part = s.distance - y_diff;
        let min_x = s.coord.x - x_part;
        let max_x = s.coord.x + x_part;
        Some((min_x, max_x))
    }).collect();

    sensors.sort_by(|lhs, rhs| {
        lhs.0.cmp(&rhs.0)
    });

    let mut ranges: Vec<Range<i64>>= vec![];
    let mut current_x = lower_limit_x;
    for (min_x, max_x) in sensors.iter() {        
        let start = i64::max(*min_x, current_x);                
        let end = i64::min(*max_x, upper_limit_x);
        if end < current_x || start > upper_limit_x {
            continue;
        }
        ranges.push(start..end + 1);
        current_x = end + 1;
    }
    ranges
}

fn find_useen_point(
    sensors: &Vec<Sensor>,
    min: C,
    max: C
) -> Option<C> {
    let mut quadrant_stack = vec![(min, max)];
    while let Some((min, max)) = quadrant_stack.pop() {
        if min == max {
            if sensors.iter().all(|pair| {
                !pair.within_sensor_range(&min)
            }) {
                return Some(min);
            }
        } else {
            let mid = Coord::new((min.x + max.x) / 2, (min.y + max.y) / 2);
            let quadrants = [
                (min, mid),
                (Coord::new(mid.x + 1, min.y), Coord::new(max.x, mid.y)),
                (Coord::new(min.x, mid.y + 1), Coord::new(mid.x, max.y)),
                (Coord::new(mid.x + 1, mid.y + 1), max),
            ];
            for quadrant in quadrants {
                if quadrant.0.x > quadrant.1.x || quadrant.0.y > quadrant.1.y {
                    continue;
                }

                if sensors
                    .iter()
                    .all(|pair| {
                        pair.can_contain_unseen_points(quadrant.0, quadrant.1)
                    }) {
                    quadrant_stack.push(quadrant);
                }
            }
        }
    }
    None
}