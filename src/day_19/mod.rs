use std::str::FromStr;
use anyhow::{Result, Error, anyhow};
use std::collections::HashMap;
use std::cmp::Ord;
pub struct NotEnoughMinerals {
    blueprints: Vec<HashMap<Mineral, HashMap<Mineral, usize>>>
}

impl crate::Advent for NotEnoughMinerals {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let blueprints = data.lines().map(|l| {
            let (_, robot_strings) = l.split_once(": ").unwrap();
            let robot_strings = robot_strings.strip_suffix(".").unwrap();
            robot_strings.split(". ").map(|rs| {            
                let words: Vec<_> = rs.split(" ").collect();
                let mines = Mineral::from_str(words[1]).unwrap();
                let mut robots: HashMap<Mineral, usize> = HashMap::new();
                
                let mut costs_index = 4;
                while costs_index < words.len() {
                    robots.insert(
                        Mineral::from_str(words[costs_index + 1]).unwrap(),
                        words[costs_index].parse().unwrap()
                    );
                    costs_index += 3;
                }
                (mines, robots)
            }).collect()
        }).collect();
        Self { blueprints }
    }
    
    fn part_01(&self) -> String {
        let mut quality_levels_sum = 0;
        let time_limit = 24;
        for (i, blueprint) in self.blueprints.iter().enumerate() {
            let factory = Factory::new(blueprint.clone());              
            let max_geodes = find_best_strategy(&factory, time_limit, 0);
            let quality_level = (i + 1) * max_geodes;
            quality_levels_sum += quality_level;
        }
        
        quality_levels_sum.to_string()
    }

    fn part_02(&self) -> String {
        let time_limit = 32;
        let mut result = 1;
        let limit = std::cmp::min(self.blueprints.len(), 3);
        for blueprint in self.blueprints[0..limit].iter() {
            let factory = Factory::new(blueprint.clone());              
            let max_geodes = find_best_strategy(&factory, time_limit, 2);
            result *= max_geodes;            
        }
        result.to_string()
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Mineral {
    Ore,
    Clay, 
    Obsidian,
    Geode
}

impl FromStr for Mineral {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(Self::Ore),
            "clay" => Ok(Self::Clay),
            "obsidian" => Ok(Self::Obsidian),
            "geode" => Ok(Self::Geode),
            _ => Err(anyhow!("Invalid string for a material: {}", s))
        }
    }
}

#[derive(Clone, Debug)]
struct MineralState {
    robots: usize, // Current robot count
    minerals: usize, // Current mineral count
    prices: HashMap<Mineral, usize>,
}

impl MineralState {
    fn new(prices: HashMap<Mineral, usize>) -> Self {
        Self {            
            prices,
            robots: 0,
            minerals: 0,
        }
    }
}

#[derive(Clone)]
struct Factory {
    mineral_states: HashMap<Mineral, MineralState>,
    time_passed: usize,
}

impl Factory {
    fn new(blueprint: HashMap<Mineral, HashMap<Mineral, usize>>) -> Self {
        let mut mineral_states: HashMap<Mineral, MineralState> = HashMap::new();
        for (m, costs) in blueprint {
            mineral_states.insert(m, MineralState::new(costs));
        }
        mineral_states.get_mut(&Mineral::Ore).unwrap().robots = 1;
        Self {
            mineral_states,
            time_passed: 0,
        }
    }

    fn state(&self, mineral: &Mineral) -> &MineralState {
        &self.mineral_states[mineral]
    }

    fn state_mut(&mut self, mineral: &Mineral) -> &mut MineralState {
        self.mineral_states.get_mut(mineral).unwrap()
    }

    fn time_till_buy(&self, mineral: &Mineral) -> usize {
        // In how many minutes we can buy a robot of mineral type taking into account 
        // the current resources we have.

        let state = self.state(&mineral);              
        let max_minutes_till_buy = state.prices.iter().map(|(m, p)| {
            let prev_state = self.state(&m);
            if prev_state.minerals >= *p {
                0
            } else {
                let remainder = *p - prev_state.minerals;
                (remainder + prev_state.robots - 1) / prev_state.robots
            }            
        }).max().unwrap();
        max_minutes_till_buy + 1
    }

    fn pass_minute(&mut self, new_robot: Option<&Mineral>) {
        {
            if let Some(robot) = new_robot {
                self.pay_for_robot(robot);
            }
        }
        
        {
            self.mine_resources();
        }
        

        if let Some(robot) = new_robot {
            self.add_robot(robot);
        }
        
        self.time_passed += 1;
    }

    fn mine_resources(&mut self) {
        for state in self.mineral_states.values_mut() {
            state.minerals += state.robots;
        }
    }
    
    fn add_robot(&mut self, robot: &Mineral) {
        let state = self.state_mut(robot);
        state.robots += 1;
    }

    fn pay_for_robot(&mut self, robot: &Mineral) {
        for (m, price) in self.state(robot).prices.clone() {
            self.state_mut(&m).minerals -= price;
        }
    }

    fn could_buy(&self) -> Vec<Mineral> {
        self.mineral_states.iter().filter_map(|(mineral, state)| {
            if state.prices.iter().all(|(m, _)| {
                self.mineral_states[m].robots > 0
            }) {
                Some(*mineral)
            } else {
                None
            }
        }).collect()        
    }
}

impl std::fmt::Display for Factory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "-----------------------")?;
        writeln!(f, "Passed minute: {}, State:", self.time_passed)?;
        for (m, s) in &self.mineral_states {
            writeln!(f, "{: <8} -> R: {: >2}, M: {: >2}", format!("{:?}", m), s.robots, s.minerals)?;
        }
        write!(f, "\n")
    }
}

fn find_best_strategy(initial_factory: &Factory, time_limit: usize, max_diviation: usize) -> usize {
    // Mineral to buy, time to buy, Factory
    let mut factories: Vec<(Mineral, usize, Factory)> = Vec::new();
    let mut max_geodes: usize = 0;
    let mut could_buy = initial_factory.could_buy();
    could_buy.sort_unstable();
    for mineral in could_buy {
        let time_till_buy = initial_factory.time_till_buy(&mineral);
        factories.push((mineral, time_till_buy, initial_factory.clone()));
    }

    let mut max_geodes_at_time: HashMap<usize, usize> = HashMap::new();

    while let Some((mineral, time_till_buy, mut factory)) = factories.pop() {        
        let pass_time = std::cmp::min(time_till_buy, time_limit - (factory.time_passed));
        let mut bought_robot = false;
        for minute in 1..=pass_time {
            if minute == time_till_buy {
                bought_robot = true;
                factory.pass_minute(Some(&mineral));
            } else {
                factory.pass_minute(None)
            }                
        }

        let current_max_geodes = factory.state(&Mineral::Geode).minerals;
        if factory.time_passed >= time_limit {   
            if current_max_geodes > max_geodes {                
                max_geodes = current_max_geodes;                
            }
            continue;
        }
        let max_geodes_at_the_time = max_geodes_at_time.entry(factory.time_passed).or_default();
        if current_max_geodes + max_diviation < *max_geodes_at_the_time {
            continue;
        } else if current_max_geodes > *max_geodes_at_the_time {
            *max_geodes_at_the_time = current_max_geodes;
        }

        if bought_robot {
            let mut could_buy = factory.could_buy();
            could_buy.sort_unstable();
            for m in could_buy.into_iter() {                        
                let ttb = factory.time_till_buy(&m);
                if ttb == 0 {
                    println!("TTB = 0! {}\nM: {:?}", factory, m);
                    continue;
                }
                factories.push((m, ttb, factory.clone()))
            }
        }
    }
    max_geodes
}