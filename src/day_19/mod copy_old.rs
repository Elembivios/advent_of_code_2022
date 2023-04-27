use std::str::FromStr;
use anyhow::{Result, Error, anyhow};
use owo_colors::OwoColorize;
use std::collections::HashMap;

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
                let mut robots: RobotPrices = HashMap::new();
                
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
        let blueprint = &self.blueprints[0];
        let mut factory = Factory::new(blueprint.clone());
        
        while factory.time_remaining != 0 {
            factory.pass_minute();
        }
        1.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
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

//
type RobotPrices = HashMap<Mineral, usize>;

#[derive(Clone)]
struct Factory {
    blueprint: HashMap<Mineral, RobotPrices>,
    minerals: HashMap<Mineral, usize>, // Current minerals count
    robots: HashMap<Mineral, usize>, // Current robot count
    time_limit: usize,
    time_remaining: usize
}

impl Factory {
    fn new(blueprint: HashMap<Mineral, RobotPrices>) -> Self {
        let mut robots = HashMap::from_iter(
            MINERALS.into_iter().map(|m| (m, 0usize))
        );
        *robots.get_mut(&Mineral::Ore).unwrap() = 1;
        let minerals = HashMap::from_iter(
            MINERALS.into_iter().map(|m| (m, 0usize))
        );
        Self {
            blueprint, 
            minerals,
            robots,
            time_limit: 24,
            time_remaining: 24
        }
    }

    fn pass_minute(&mut self) {
        println!("------------------------");
        println!("Passed minute: {}", self.time_limit - self.time_remaining + 1);
        println!("Resources: {:?}", self.minerals);
        println!("Robots: {:?}", self.robots);

        let mut bought_robots: HashMap<Mineral, usize> = HashMap::new();

        while let Some(best_to_buy) = self.get_best_robot_to_buy() {
            *bought_robots.entry(best_to_buy).or_default() += 1;
            self.pay_for_robot(&best_to_buy);
        }

        self.mine_resources();

        self.add_robots(bought_robots);

        self.time_remaining -= 1;
    }

    fn get_best_robot_to_buy(&self) -> Option<Mineral> {
        // Returns a robot if this is the right time to buy one
        // Returns a None if we should wait
        let can_buy_robots: Vec<Mineral> = MINERALS.into_iter().filter_map(|m| {
            if self.can_buy_robot(&m) {
                Some(m)
            } else {
                None
            }
        }).collect();     

        if can_buy_robots.len() == 0 {
            println!("No robots to buy..");
            return None;
        }

        // Do we wait to buy it or we buy a lesser one
        println!("Can buy robots: {:?}", can_buy_robots);
        let mut current_best_score = self.get_num_robots_can_buy();
        let mut wait = true;

        for robot in can_buy_robots.iter() {
            let mut factory_clone = self.clone();    
            factory_clone.pay_for_robot(robot);
            factory_clone.mine_resources();
            let new_robots = HashMap::from([(*robot, 1usize)]);
            factory_clone.add_robots(new_robots);
            factory_clone.time_remaining -= 1;
            let new_score = factory_clone.get_num_robots_can_buy();
            if new_score[&Mineral::Obsidian] > current_best_score[&Mineral::Obsidian] {
                println!("Found new best for obsidian: {} -> {}", current_best_score[&Mineral::Obsidian], new_score[&Mineral::Obsidian]);
                current_best_score = new_score;
                wait = false;
            }
            // for (m, score) in new_score {
            //     if score > current_best_score[&m] {
            //         println!("Found new best for {:?} ({} -> {})", m, current_best_score[&m], score);
            //         *current_best_score.get_mut(&m).unwrap() = score;
            //     }
            // }
        }
        println!("Best scores: {:?}, Wait: {}", current_best_score, wait);
        if !wait {
            let (best_to_buy, _score) = self.get_best_robot_score(&current_best_score);

            println!("Best to buy: {:?}", best_to_buy);

            if can_buy_robots.contains(best_to_buy) {
                return Some(*best_to_buy)
            } else {            
                // We wait with the purchase
                return None
            }
        }
        return None        
    }

    fn can_buy_robot(&self, mineral: &Mineral) -> bool {
        let prices = &self.blueprint[mineral];
        let can_buy = prices.iter().all(|(m, price)| {
            self.minerals[m] >= *price
        });
        can_buy
    }

    fn pay_for_robot(&mut self, mineral: &Mineral) {
        for (m, price) in &self.blueprint[mineral] {
            *self.minerals.get_mut(m).unwrap() -= *price
        }
    }

    fn add_robots(&mut self, robots: HashMap<Mineral, usize>) {
        for (robot_type, count) in robots {        
            *self.robots.entry(robot_type).or_default() += count;
        }
    }

    fn mine_resources(&mut self) {
        for (robot_type, count) in &self.robots {
            *self.minerals.get_mut(robot_type).unwrap() += count;
        }
    }

    fn time_and_sequences_till_buy(&self) -> HashMap<Mineral, (usize, usize)> {        
        let mut time_and_sequences = HashMap::new();
        for (i, mineral) in MINERALS.iter().enumerate() {
            let prices = &self.blueprint[mineral];
            let (_, time_and_sequence) = prices.iter().map(|(m, price)| {
                let remaining_price = if *price > self.minerals[m] {
                    *price - self.minerals[m]
                } else {
                    0
                };
                    
                let minutes_to_buy = if self.robots[m] == 0 { // For example: We don't have clay robots to pay clay for obsidian robot

                    (usize::MAX, usize::MAX)

                } else {
                    (
                        // Div ceil         
                        (remaining_price + self.robots[m] - 1) / self.robots[m],                    
                        (*price + self.robots[m] - 1) / self.robots[m],
                    )    
                };
                // let minutes_to_buy = (remaining_price + self.robots[m] as isize - 1) / self.robots[m] as isize; // Div ceil            
                (m, minutes_to_buy)
            }).max_by(|a, b| a.1.0.cmp(&b.1.0)).unwrap();        
            time_and_sequences.insert(*mineral, time_and_sequence);
        }
        time_and_sequences
    }

    fn get_num_robots_can_buy(&self) -> HashMap<Mineral, f32> {
        let time_and_sequences = self.time_and_sequences_till_buy();
        let mut num_robots_can_buy = HashMap::new();
        for (m, (time, sequence)) in time_and_sequences {
            if time > self.time_remaining {
                num_robots_can_buy.insert(m, 0.0);
                continue;
            }
            let nr = (self.time_remaining + time) as f32 / sequence as f32 + 1.0;
            num_robots_can_buy.insert(m, nr);
        }

        num_robots_can_buy
    }

    fn get_best_robot_score<'a>(&self, robots_score: &'a HashMap<Mineral, f32>) -> (&'a Mineral, &'a f32) {

        let max = robots_score.iter().max_by(|a, b| {
            a.1.partial_cmp(&b.1).unwrap()
        }).unwrap();
        max
    }


    fn get_best_robot_to_buy2(&self) {
        let mut mineral_prices_sum: HashMap<Mineral, HashMap<Mineral, usize>> = HashMap::new();
        for mineral in MINERALS.iter() {
            
        }
    }
}