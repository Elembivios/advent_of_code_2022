use std::str::FromStr;
use anyhow::{Result, Error, anyhow};
use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::cmp::Ord;

use crate::utils::wait_user_input;

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
        let mut best_blueprint = (0, 0);
        for (i, blueprint) in self.blueprints.iter().enumerate() {
            println!("Searching blueprint: {}", i);
            let mut factory = Factory::new(blueprint.clone());              
            let mut max_geode = 0;  
            factory.run(&mut max_geode);
            if max_geode > best_blueprint.1 {
                best_blueprint = (i + 1, max_geode);
            }
        }
        
        let best_blueprint_score = best_blueprint.0 * best_blueprint.1;
        best_blueprint_score.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Mineral {
    Ore,
    Clay, 
    Obsidian,
    Geode
}


// static MINERALS: [Mineral; 4] = [Mineral::Ore, Mineral::Clay, Mineral::Obsidian, Mineral::Geode];

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
    prices: HashMap<Mineral, usize>
}

impl MineralState {
    fn new(prices: HashMap<Mineral, usize>) -> Self {
        Self {            
            prices,
            robots: 0,
            minerals: 0
        }
    }

    fn can_buy(&self, other_states: &HashMap<Mineral, MineralState>) -> bool {
        // Can we buy the robot now
        self.prices.iter().all(|(m, price)| {
            other_states[m].minerals >= *price
        })
    }

    fn add_robots(&mut self, count: usize) {
        self.robots += count;
    }

    fn mine(&mut self) {
        self.minerals += self.robots;
    }
}


#[derive(Clone)]
struct Factory {
    index: usize,
    mineral_states: HashMap<Mineral, MineralState>,
    time_limit: usize,
    time_remaining: usize,
    lock_buy_of: Vec<Mineral>
}

impl Factory {
    fn new(blueprint: HashMap<Mineral, HashMap<Mineral, usize>>) -> Self {
        let mut mineral_states: HashMap<Mineral, MineralState> = HashMap::new();
        for (m, costs) in blueprint {
            mineral_states.insert(m, MineralState::new(costs));
        }
        mineral_states.get_mut(&Mineral::Ore).unwrap().robots = 1;
        Self {
            index: 0,
            mineral_states,
            time_limit: 24,
            time_remaining: 24,
            lock_buy_of: vec![]
        }
    }

    fn state(&self, mineral: &Mineral) -> &MineralState {
        &self.mineral_states[mineral]
    }

    fn state_mut(&mut self, mineral: &Mineral) -> &mut MineralState {
        self.mineral_states.get_mut(mineral).unwrap()
    }

    fn run(&mut self, max_geode: &mut usize) {        
        while self.time_remaining > 0 {
            let can_buy: Vec<Mineral> = self.mineral_states.iter().filter_map(|(m, s)| {
                if s.can_buy(&self.mineral_states) {
                    Some(*m)
                } else {
                    None
                }
            }).collect();
    
            if can_buy.len() > 0 {
                // println!("{}", self);
                // println!("Can buy: {:?}", can_buy);
                // println!("Max geode: {:?}", max_geode);
                // wait_user_input();
                for m in can_buy.into_iter() {
                    if self.lock_buy_of.contains(&m) {
                        continue;
                    }
                    let mut clon = self.clone();
                    let to_buy = if clon.time_limit > 1 {
                        vec![m]
                    } else {
                        vec![]
                    };
                    clon.pass_minute(to_buy);
                    clon.lock_buy_of = vec![];
                    clon.index += 1;
                    clon.run(max_geode);
                    let new_geode =  clon.mineral_states[&Mineral::Geode].minerals;
                    if new_geode > *max_geode {
                        // println!("{}", self);
                        // println!("{}", clon);
                        // println!("New max geode: {} -> {}", max_geode, new_geode);
                        // wait_user_input();
                        *max_geode = new_geode;
                    }
                    self.lock_buy_of.push(m);
                }
            }
            self.pass_minute(vec![]);
        }

        // self.mine_resources();

        let geode_count = self.mineral_states[&Mineral::Geode].minerals;
        if geode_count > *max_geode {
            // println!("{}", self);
            // println!("New max geode: {} -> {}", max_geode, geode_count);
            // wait_user_input();
            *max_geode = geode_count;
        }
    }

    fn pass_minute(&mut self, buy_robots: Vec<Mineral>) {
        for robot in &buy_robots {
            let prices = self.state(robot).prices.clone();
            for (m, price) in prices {
                self.state_mut(&m).minerals -= price;
            }
        }

        self.mine_resources();

        for robot in &buy_robots {
            self.state_mut(robot).add_robots(1);
        }
        
        self.time_remaining -= 1;
    }

    fn mine_resources(&mut self) {
        for state in self.mineral_states.values_mut() {
            state.mine();
        }
    }

    // fn get_best_robot_to_buy(&self) -> Option<Mineral> {
    //     // Returns a robot if this is the right time to buy one
    //     // Returns a None if we should wait

    //     let can_buy_robots: Vec<Mineral> = self.mineral_states.iter().filter_map(|(m, s)| {
    //         if s.can_buy() {
    //             Some(*m)
    //         } else {
    //             None
    //         }
    //     }).collect();

    //     if can_buy_robots.len() == 0 {
    //         println!("No robots to buy..");
    //         return None;
    //     }

    //     // Do we wait to buy it or we buy a lesser one
    //     println!("Can buy robots: {:?}", can_buy_robots);
    //     let mut current_best_score = self.get_num_robots_can_buy();
    //     let mut wait = true;

    //     for robot in can_buy_robots.iter() {
    //         let mut factory_clone = self.clone();    
    //         factory_clone.pay_for_robot(robot);
    //         factory_clone.mine_resources();
    //         let new_robots = HashMap::from([(*robot, 1usize)]);
    //         factory_clone.add_robots(new_robots);
    //         factory_clone.time_remaining -= 1;
    //         let new_score = factory_clone.get_num_robots_can_buy();
    //         if new_score[&Mineral::Obsidian] > current_best_score[&Mineral::Obsidian] {
    //             println!("Found new best for obsidian: {} -> {}", current_best_score[&Mineral::Obsidian], new_score[&Mineral::Obsidian]);
    //             current_best_score = new_score;
    //             wait = false;
    //         }
    //         // for (m, score) in new_score {
    //         //     if score > current_best_score[&m] {
    //         //         println!("Found new best for {:?} ({} -> {})", m, current_best_score[&m], score);
    //         //         *current_best_score.get_mut(&m).unwrap() = score;
    //         //     }
    //         // }
    //     }
    //     println!("Best scores: {:?}, Wait: {}", current_best_score, wait);
    //     if !wait {
    //         let (best_to_buy, _score) = self.get_best_robot_score(&current_best_score);

    //         println!("Best to buy: {:?}", best_to_buy);

    //         if can_buy_robots.contains(best_to_buy) {
    //             return Some(*best_to_buy)
    //         } else {            
    //             // We wait with the purchase
    //             return None
    //         }
    //     }
    //     return None        
    // }



    // fn time_and_sequences_till_buy(&self) -> HashMap<Mineral, (usize, usize)> {        
    //     let mut time_and_sequences = HashMap::new();
    //     for (i, mineral) in MINERALS.iter().enumerate() {
    //         let prices = &self.blueprint[mineral];
    //         let (_, time_and_sequence) = prices.iter().map(|(m, price)| {
    //             let remaining_price = if *price > self.minerals[m] {
    //                 *price - self.minerals[m]
    //             } else {
    //                 0
    //             };
                    
    //             let minutes_to_buy = if self.robots[m] == 0 { // For example: We don't have clay robots to pay clay for obsidian robot

    //                 (usize::MAX, usize::MAX)

    //             } else {
    //                 (
    //                     // Div ceil         
    //                     (remaining_price + self.robots[m] - 1) / self.robots[m],                    
    //                     (*price + self.robots[m] - 1) / self.robots[m],
    //                 )    
    //             };
    //             // let minutes_to_buy = (remaining_price + self.robots[m] as isize - 1) / self.robots[m] as isize; // Div ceil            
    //             (m, minutes_to_buy)
    //         }).max_by(|a, b| a.1.0.cmp(&b.1.0)).unwrap();        
    //         time_and_sequences.insert(*mineral, time_and_sequence);
    //     }
    //     time_and_sequences
    // }

    // fn get_num_robots_can_buy(&self) -> HashMap<Mineral, f32> {
    //     let time_and_sequences = self.time_and_sequences_till_buy();
    //     let mut num_robots_can_buy = HashMap::new();
    //     for (m, (time, sequence)) in time_and_sequences {
    //         if time > self.time_remaining {
    //             num_robots_can_buy.insert(m, 0.0);
    //             continue;
    //         }
    //         let nr = (self.time_remaining + time) as f32 / sequence as f32 + 1.0;
    //         num_robots_can_buy.insert(m, nr);
    //     }

    //     num_robots_can_buy
    // }
}

impl std::fmt::Display for Factory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "----------{}-------------", self.index)?;
        writeln!(f, "Passed minute: {}, State:", self.time_limit - self.time_remaining + 1)?;
        for (m, s) in &self.mineral_states {
            writeln!(f, "{: <8} -> R: {: >2}, M: {: >2}", format!("{:?}", m), s.robots, s.minerals)?;
        }
        write!(f, "\n")
    }
}