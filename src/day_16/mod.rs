use std::collections::HashMap;
use anyhow::{Error, Context};
use std::str::FromStr;

pub struct ProboscideaVolcanium {
    valves: Vec<Valve>,
    map: HashMap<String, Vec<String>>
}

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
}

impl FromStr for Valve {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Valve ").context("Invalid valve string")?;
        let (name, rest) = s.split_at(2);
        let val = rest.strip_prefix(" has flow rate=").context("Invalid valve string")?.parse().context("Can't parse value to integer")?;
        Ok(Valve {
            name: name.to_string(),
            flow_rate: val
        })
    }
}

impl crate::Advent for ProboscideaVolcanium {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let valves: Vec<Valve> = data.lines().map(|l| {
            let (lhs, rhs) = l.split_once("; ").unwrap();
            let valve = Valve::from_str(lhs).unwrap();
            let leads_to: Vec<String> = rhs.strip_prefix("tunnels lead to valves ").unwrap().split(", ").map(|s| s.to_owned()).collect();

            map.insert(valve.name.clone(), leads_to);
            valve
        }).collect();

        ProboscideaVolcanium { valves, map }
    }

    fn part_01(&self) -> String {
        let mut valves = self.valves.clone();
        valves.sort_by(|a, b| { a.flow_rate.cmp(&b.flow_rate)});

        1.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}
