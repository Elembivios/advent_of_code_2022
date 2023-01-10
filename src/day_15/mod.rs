use std::{ops::Range};

use crate::utils::point::Coord;

type C = Coord<i64>;

struct SensorBeaconPair {
    sensor: C,
    beacon: C,
    distance: i64
}

// impl SensorBeaconPair {
//     fn new(sensor: C, beacon: C) -> Self {
//         let distance = sensor.manhattan_distance(&beacon);
//         SensorBeaconPair { sensor, beacon, distance }
//     }

//     fn within_sensor_range(&self, other: &C) -> bool {
//         self.distance <= other.manhattan_distance(&self.sensor)
//     }

//     fn can_contain_unseen_points(&self, min: C, max: C) -> bool {
//         let corners = [
//             Coord::new(min.x, min.y),
//             Coord::new(min.x, max.y),
//             Coord::new(max.x, min.y),
//             Coord::new(max.x, max.y),
//         ];
//         let largest_dist = corners.into_iter().map(|corner| {
//             corner.manhattan_distance(&self.sensor)
//         }).max().unwrap();
//         largest_dist > self.distance
//     }
// }

pub struct BeaconExclusionZone {
    sensors_and_beacons: Vec<(C, C, i64)>
}

impl crate::Advent for BeaconExclusionZone {
    fn new(data: &str) -> Self
    where 
    Self: Sized 
    {
        let sensors_and_beacons: Vec<(C, C, i64)> = data.lines().map(|l| {
            let (lhs, rhs) = l.split_once(": ").unwrap();
            let lhs: Vec<i64> = lhs.strip_prefix("Sensor at ").unwrap().split(", ").map(|part| {
                let value = part.split("=").last().unwrap();
                value.parse().unwrap()
            }).collect();
            let rhs: Vec<i64> = rhs.strip_prefix("closest beacon is at ").unwrap().split(", ").map(|part| {
                let value = part.split("=").last().unwrap();
                value.parse().unwrap()
            }).collect();
            let sensor = Coord::new(lhs[0], lhs[1]);
            let beacon = Coord::new(rhs[0], rhs[1]);
            let distance = sensor.manhattan_distance(&beacon);

            (sensor, beacon, distance)
        }).collect();  
        BeaconExclusionZone { sensors_and_beacons }
    }
    fn part_01(&self) -> String {
        let y = 2_000_000;
        // let y = 10; // For example input
        let lower_limit_x = self.sensors_and_beacons.iter().map(|(s, _b, d)| {
            s.x - d
        }).min().unwrap();
        let upper_limit_x = self.sensors_and_beacons.iter().map(|(s, _b, d)| {
            s.x + d
        }).max().unwrap();
        let full_ranges = beacon_covered_at_y(&self.sensors_and_beacons, y, lower_limit_x, upper_limit_x);
        let mut to_exclude: Vec<i64> = self.sensors_and_beacons.iter().map(|(s, b, _d)| {
            vec![b, s]
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
        let mut empty_spaces: Vec<C> = vec![];
        for y in lower_limit..=upper_limit {
            let ranges = beacon_covered_at_y(&self.sensors_and_beacons, y, lower_limit, upper_limit);
            let mut current_x = lower_limit;
            for range in ranges {
                if range.start - current_x > 0 {
                    let empty = Coord::new(current_x, y);
                    empty_spaces.push(empty);
                }
                current_x = range.end;
            }
        }
        
        let empty_space = empty_spaces[0];
        let result = (empty_space.x * 4_000_000) + empty_space.y;
        result.to_string()
    }
}

fn beacon_covered_at_y(sensors_and_beacons: &Vec<(C, C, i64)>, y: i64, lower_limit_x: i64, upper_limit_x: i64) -> Vec<Range<i64>> {
    let mut sensors_and_beacons: Vec<_> = sensors_and_beacons.iter().filter_map(|(s, _b, d)| {
        let y_diff = s.y.abs_diff(y) as i64;
        if y_diff > *d {
            return None;
        }
        let x_part = d - y_diff;
        let min_x = s.x - x_part;
        let max_x = s.x + x_part;
        Some((min_x, max_x))
    }).collect();

    sensors_and_beacons.sort_by(|lhs, rhs| {
        lhs.0.cmp(&rhs.0)
    });

    let mut ranges: Vec<Range<i64>>= vec![];
    let mut current_x = lower_limit_x;
    for (min_x, max_x) in sensors_and_beacons.iter() {        
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

// fn find_useen_point(
//     sensors_and_beacons: Vec<(C, C, i64)>,
//     min: C,
//     max: C
// ) -> Option<C> {
//     let mut quadrant_stack = vec![(min, max)];
//     while let Some((min, max)) = quadrant_stack.pop() {
//         if min == max {
//             if sensors_and_beacons.iter().all(|pair| {

//             })
//         }
//     }


//     None
// }