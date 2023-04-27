use std::str::FromStr;
use std::time;
use anyhow::{Result, Error, anyhow};
use std::collections::HashMap;
use std::cmp::{Ord, Ordering};

// 400 -- too low
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
        1.to_string()
        // let mut quality_levels_sum = 0;
        // for (i, blueprint) in self.blueprints.iter().enumerate() {
        //     println!("Searching blueprint: {}", i);
        //     let mut factory = Factory::new(blueprint.clone(), 24);              
        //     let mut max_geode = 0;  
        //     factory.run(&mut max_geode);
        //     let quality_level = (i + 1) * max_geode;
        //     println!("{} ({}) -> {}", i, max_geode, quality_level);
        //     quality_levels_sum += quality_level;
        // }
        
        // quality_levels_sum.to_string()
    }

    fn part_02(&self) -> String {
        let mut quality_levels_sum = 0;
        for (i, blueprint) in self.blueprints[0..3].iter().enumerate() {
            println!("Searching blueprint: {}", i);
            let mut factory = Factory::new(blueprint.clone(), 32);              
            let mut max_geode = 0;  
            factory.run(&mut max_geode);
            let quality_level = (i + 1) * max_geode;
            println!("{} ({}) -> {}", i, max_geode, quality_level);
            quality_levels_sum += quality_level;
        }
        
        quality_levels_sum.to_string()
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Mineral {
    Ore,
    Clay, 
    Obsidian,
    Geode
}


static MINERALS: [Mineral; 4] = [Mineral::Ore, Mineral::Clay, Mineral::Obsidian, Mineral::Geode];

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

    fn minutes_to_buy(&self, quantity: usize) -> usize {
        if self.robots == 0 {
            return usize::MAX;
        }
        if self.minerals >= quantity {
            return 0;
        }
        let remaining = quantity - self.minerals;

        // Ceil division
        (remaining + self.robots - 1) / self.robots
    }

    fn sequence_to_buy(&self, quantity: usize) -> usize {
        if self.robots == 0 {
            return usize::MAX;
        } else if self.minerals >= quantity {
            return 0;
        } else {
            // Ceil division
            (quantity + self.robots - 1) / self.robots
        }
    }
}

#[derive(Debug, PartialEq)]
struct Score {
    nr_geodes: f32,
    time_to_buy: usize,
    sequence_to_buy: usize,
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp = self.nr_geodes.partial_cmp(&other.nr_geodes);
        let Some(cmp) = cmp else {
            return None;
        };
        let mut cmp = cmp;

        if cmp.is_eq() {
            cmp = self.time_to_buy.cmp(&other.time_to_buy).reverse();
            if cmp.is_eq() {
                cmp = self.sequence_to_buy.cmp(&other.sequence_to_buy).reverse();
            }
        }
        Some(cmp)
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
    fn new(blueprint: HashMap<Mineral, HashMap<Mineral, usize>>, time_limit: usize) -> Self {
        let mut mineral_states: HashMap<Mineral, MineralState> = HashMap::new();
        for (m, costs) in blueprint {
            mineral_states.insert(m, MineralState::new(costs));
        }
        mineral_states.get_mut(&Mineral::Ore).unwrap().robots = 1;
        Self {
            index: 0,
            mineral_states,
            time_limit,
            time_remaining: time_limit,
            lock_buy_of: vec![]
        }
    }

    fn state(&self, mineral: &Mineral) -> &MineralState {
        &self.mineral_states[mineral]
    }

    fn state_mut(&mut self, mineral: &Mineral) -> &mut MineralState {
        self.mineral_states.get_mut(mineral).unwrap()
    }

    fn geode_price(&self) -> HashMap<Mineral, usize> {
        let mut geode_price: HashMap<Mineral, usize> = HashMap::new();


        geode_price
    }

    fn next_to_buy(&self) {

    }

    fn could_buy(&self) -> Vec<Mineral> {
        MINERALS.into_iter().filter(|mineral| {
            self.state(mineral).prices.iter().all(|(m, _p)| {
                self.state(m).robots > 0
            })
        }).collect()        
    }

    fn mineral_minutes_to_buy(&self, mineral: &Mineral) -> usize {
        // In how many minutes we can buy a robot of mineral type taking into account 
        // the current resources we have.
        let state = self.state(&mineral);

        let max_minutes_to_buy = state.prices.iter().map(|(m, p)| {
            let mut minutes_to_buy = self.state(m).minutes_to_buy(*p);
            if minutes_to_buy == usize::MAX {
                minutes_to_buy = self.mineral_minutes_to_buy(m) + 1 + *p;
            }
            (m, minutes_to_buy)
        }).max_by(|a, b| {
            a.1.cmp(&b.1)
        }).unwrap();
        // println!("M {:?} -> {:?}", mineral, max_minutes_to_buy);
        max_minutes_to_buy.1
    }

    fn sequence_to_buy(&self, mineral: &Mineral) -> usize {
        // In how many minutes we can buy a robot of mineral type without taking 
        // into account the current resources we have.
        let state = self.state(&mineral);

        let max_sequence_to_buy = state.prices.iter().map(|(m, p)| {
            let mut sequence_to_buy = self.state(m).sequence_to_buy(*p);
            if sequence_to_buy == usize::MAX {
                sequence_to_buy = self.sequence_to_buy(m) + 1 + *p;
            }
            (m, sequence_to_buy)
        }).max_by(|a, b| {
            a.1.cmp(&b.1)
        }).unwrap();

        max_sequence_to_buy.1
    }


    // fn calculated_price(&self, mineral: &Mineral) -> (usize, usize) {
    //     let state = self.state(&mineral);

    //     let max_minutes_to_buy = state.prices.iter().map(|(m, p)| {
    //         let mut minutes_to_buy = self.state(m).minutes_to_buy(*p);

    //         if minutes_to_buy == usize::MAX {
    //             minutes_to_buy = self.mineral_minutes_to_buy(m) + 1 + *p;
    //         }

    //         (m, minutes_to_buy)
    //     }).max_by(|a, b| {
    //         a.1.cmp(&b.1)
    //     }).unwrap();

    //     max_minutes_to_buy.1
    // }
    
    fn b_line_num_geodes_we_can_buy(&self) -> f32 {
        let minutes_to_buy = self.mineral_minutes_to_buy(&Mineral::Geode);
        let sequence_to_buy = self.sequence_to_buy(&Mineral::Geode);
        // if minutes_to_buy >= self.time_remaining { // If we buy last minute it will do us no good
        //     return 0f32;
        // }
        let result = 1f32;
        let remaining_time = self.time_remaining as isize - minutes_to_buy as isize;
        result + (remaining_time as f32 / sequence_to_buy as f32)
    }

    fn calculate_geode_minutes_to_buy(&self) -> usize {
        // Calculates how many geodes will we be able to buy with current income
        self.mineral_minutes_to_buy(&Mineral::Geode)
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

    fn run_w(&mut self, max_geode: &mut usize) {
        while self.time_remaining > 0 {
            let can_buy: Vec<Mineral> =self.mineral_states
                .iter()
                .filter_map(|(m, s)| {
                    if s.can_buy(&self.mineral_states) {
                        Some(*m)
                    } else {
                        None
                    }
                }).collect();
            println!("Can buy: {:?}", can_buy);            
            if can_buy.len() > 0 && self.time_limit > 1 {
                let mut clon1 = self.clone();
                clon1.pass_minute(vec![]);
                let score1 = Score { 
                    nr_geodes: clon1.b_line_num_geodes_we_can_buy(),
                    time_to_buy: clon1.calculate_geode_minutes_to_buy(),
                    sequence_to_buy: clon1.sequence_to_buy(&Mineral::Geode)
                };

                let mut scores: Vec<(Option<Mineral>, Score)> = vec![(None, score1)];

                for m in can_buy.into_iter() {                                        
                    let mut clon2 = self.clone();                    
                    clon2.pass_minute(vec![m]);
                    let score2 = Score {
                        nr_geodes: clon2.b_line_num_geodes_we_can_buy(),
                        time_to_buy: clon2.calculate_geode_minutes_to_buy(),
                        sequence_to_buy: clon2.sequence_to_buy(&Mineral::Geode)
                    };
                    scores.push((Some(m), score2));                                                        
                }
                println!("Scores: {:#?}", scores);
                let (best_robot, max_score) = scores.into_iter().max_by(|a, b| {
                    let cmp = a.1.partial_cmp(&b.1).unwrap();
                    if cmp.is_eq() {

                        a.0.cmp(&b.0)
                    } else {
                        cmp
                    }
                }).unwrap();
                
                if let Some(best_robot) = best_robot {
                    println!("Bought {:?}", best_robot);                    
                    self.pass_minute(vec![best_robot]);
                    println!("Self: {}", self);
                } else {
                    println!("Waited...");
                    self.pass_minute(vec![]);
                }
            } else {
                self.pass_minute(vec![]);
            }        
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

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;    
    lazy_static! {
        static ref BLUEPRINT: HashMap<Mineral, HashMap<Mineral, usize>> = {
            HashMap::from([
                (Mineral::Ore, HashMap::from([
                    (Mineral::Ore, 4)
                ])),
                (Mineral::Clay, HashMap::from([
                    (Mineral::Ore, 2)
                ])),
                (Mineral::Obsidian, HashMap::from([
                    (Mineral::Ore, 3),
                    (Mineral::Clay, 14),
                ])),
                (Mineral::Geode, HashMap::from([
                    (Mineral::Ore, 2),
                    (Mineral::Obsidian, 7)
                ]))
            ])            
        };
    }

    #[test]
    fn it_works() {
        let b = &*BLUEPRINT;
        let mut factory = Factory::new(b.clone(), 24);
        let minutes_to_buy = factory.mineral_minutes_to_buy(&Mineral::Geode);
        let sequence_to_buy = factory.sequence_to_buy(&Mineral::Geode);
        let nr_geodes = factory.b_line_num_geodes_we_can_buy();
        assert_eq!(minutes_to_buy, 25);
        assert_eq!(sequence_to_buy, 25);
        assert_eq!(nr_geodes, 0f32);

        factory.pass_minute(vec![]);
        factory.pass_minute(vec![]);
        factory.pass_minute(vec![Mineral::Clay]);        

        println!("{}", factory);
        let minutes_to_buy = factory.calculate_geode_minutes_to_buy();
        let sequence_to_buy = factory.sequence_to_buy(&Mineral::Geode);
        let nr_geodes = factory.b_line_num_geodes_we_can_buy();
        assert_eq!(minutes_to_buy, 22); // Passed 3 minutes - Should still B line it
        assert_eq!(sequence_to_buy, 22);
        assert_eq!(nr_geodes, 0f32);


        factory.pass_minute(vec![]);
        let mut factory_02 = factory.clone();        
        factory.pass_minute(vec![Mineral::Clay]);
        println!("{}", factory);
        // factory.pass_minute(vec![Mineral::Clay]);        
        // factory.pass_minute(vec![]);

        let minutes_to_buy = factory.calculate_geode_minutes_to_buy();
        let sequence_to_buy = factory.sequence_to_buy(&Mineral::Geode);
        let nr_geodes = factory.b_line_num_geodes_we_can_buy();
        assert_eq!(minutes_to_buy, 14);
        assert_eq!(sequence_to_buy, 15);
        assert_eq!(nr_geodes, 1.3333334);

        factory_02.pass_minute(vec![]);
        let minutes_to_buy = factory_02.calculate_geode_minutes_to_buy();
        let sequence_to_buy = factory_02.sequence_to_buy(&Mineral::Geode);
        let nr_geodes = factory_02.b_line_num_geodes_we_can_buy();
        println!("Min: {}, Seq: {}, Nr: {}", minutes_to_buy, sequence_to_buy, nr_geodes);
        assert_eq!(minutes_to_buy, 14);
        assert_eq!(sequence_to_buy, 15);
        assert_eq!(nr_geodes, 1.3333334);

        println!("{}", factory);
        println!("Minutes to buy: {}", minutes_to_buy);
    }


    #[test]
    fn part1() {
        let b = &*BLUEPRINT;
        let mut factory = Factory::new(b.clone(), 24);
        factory.run_w(&mut 0);
    }
}

