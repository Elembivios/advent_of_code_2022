use std::{str::FromStr, slice::Iter};
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
        factory.strategy();
        // for _minute in (0..24).rev() {
        //     factory.pass_minute();
        // }
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

struct Factory {
    blueprint: HashMap<Mineral, RobotPrices>,
    minerals: HashMap<Mineral, usize>, // Current minerals count
    robots: HashMap<Mineral, usize>, // Current robot count
    time_remaining: usize
}

impl Factory {
    fn new(blueprint: HashMap<Mineral, RobotPrices>) -> Self {
        let mut robots = HashMap::with_capacity(4);
        robots.insert(Mineral::Ore, 1);
        let minerals = HashMap::from_iter(
            MINERALS.into_iter().map(|m| (m, 0usize))
        );
        Self {
            blueprint, 
            minerals,
            robots,
            time_remaining: 24
        }
    }

    fn pass_minute(&mut self) {
        println!("------------------------");
        println!("Passed minute: {}", self.time_remaining);
        self.time_remaining -= 1;
        let new_robots = self.buy_robots();
        self.mine_resources();
        self.add_robots(new_robots);

    }

    fn buy_robots(&mut self) -> HashMap<Mineral, usize> {
        // Checks if we can buy new robots.
        // Spends resources to buy new robots, but doesn't add them to 'robots' 
        // attribute just yet. Instead it returns the map of which robots it has 
        // bought.     
        let mut new_robots: HashMap<Mineral, usize> = HashMap::with_capacity(4);
        for mineral in MINERALS.iter().rev() {
            let prices = self.blueprint.get_mut(mineral).unwrap();
            let can_buy = prices.iter().all(|(m, price)| {
                self.minerals[m] >= *price
            });
            if can_buy {
                println!("Buying robot for mining {:?}", mineral);
                *new_robots.entry(*mineral).or_default() += 1;
                for (m, price) in prices {
                    *self.minerals.get_mut(m).unwrap() -= *price;
                }
            }
        }
        new_robots
    }

    fn add_robots(&mut self, robots: HashMap<Mineral, usize>) {
        for (robot_type, count) in robots {        
            *self.robots.entry(robot_type).or_default() += count;
        }
    }

    fn mine_resources(&mut self) {
        for (robot_type, count) in &self.robots {
            println!("Collected {} {:?}", count, robot_type);
            *self.minerals.get_mut(robot_type).unwrap() += count;
        }
    }

    fn strategy(&mut self) {
        // Calculate which robots to buy and when
        let time_limit = 24;
        let mut time_remaining = time_limit.clone();
        
        


        // Get the materials needed for constructing one geode robot
        let mut minerals_needed: HashMap<Mineral, isize> = HashMap::from_iter(
            MINERALS.iter().map(|m| (m.clone(), 0))
        );
        for mineral in MINERALS.iter().rev() {
            for (m, price) in &self.blueprint[mineral] {
                *minerals_needed.get_mut(m).unwrap() += *price as isize - self.minerals[mineral] as isize
            }
        }
        println!("Minerals needed: {:?}", minerals_needed);
        
        let current_mining_forecast: HashMap<Mineral, usize> = HashMap::from_iter(self.robots.iter().map(|(rt, rc)| {
            (rt.clone(), rc * time_remaining)
        }));

        let max_demand = minerals_needed.iter().max_by(|a, b| {
            a.1.cmp(&b.1)
        }).unwrap();
        println!("Max demand: {:?}", max_demand);
        // let max_duration = max_demand;

    }
}