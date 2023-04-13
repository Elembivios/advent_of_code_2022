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
            factory.pass_minute(None);
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

    fn pass_minute(&mut self, force_buy: Option<Mineral>) {
        println!("------------------------");
        println!("Passed minute: {}", self.time_limit - self.time_remaining + 1);
        println!("Resources: {:?}", self.minerals);

        let mut bought_robots: HashMap<Mineral, usize> = HashMap::new();

        // if let Some(mineral) = force_buy {
        //     self.pay_for_robot(mineral);

        // }

        
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
            return None;
        }

        println!("Can buy robots: {:?}", can_buy_robots);

        // let best_robot_we_could_buy = self.get_best_robot_we_could_buy_with_current_income();
        // if can_buy_robots.contains(&best_robot_we_could_buy) {
        //     return Some(best_robot_we_could_buy);
        // }
        // println!("Best we could buy: {:?}", best_robot_we_could_buy);
        let mut robots_to_buy_score: HashMap<Mineral, usize> = HashMap::new();

        // Do we wait to buy it or we buy a lesser one
        robots_to_buy_score.insert(Mineral::Geode, self.num_can_buy(&Mineral::Geode));

        for robot in can_buy_robots.iter() {
            let mut factory_clone = self.clone();    
            factory_clone.pay_for_robot(robot);
            factory_clone.mine_resources();
            let new_robots = HashMap::from([(*robot, 1usize)]);
            factory_clone.add_robots(new_robots);
            factory_clone.time_remaining -= 1;
            robots_to_buy_score.insert(*robot, factory_clone.num_can_buy(&Mineral::Geode));
        }
        println!("Robots to buy score: {:?}", robots_to_buy_score);

        let (best_to_buy, _score) = robots_to_buy_score.iter().max_by(|a, b| {
            let c = a.1.cmp(&b.1);
            if c.is_eq() {
                let pos1 = MINERALS.iter().position(|m| m == a.0);
                let pos2 = MINERALS.iter().position(|m| m == b.0);
                return pos1.cmp(&pos2);
            }
            c
        }).unwrap();

        println!("Best to buy: {:?}", best_to_buy);

        if can_buy_robots.contains(best_to_buy) {
            return Some(*best_to_buy)
        } else {
            
            // We wait with the purchase
            return None
        }

        
    }

    // fn buy_robots(&mut self) -> HashMap<Mineral, usize> {
    //     let mut robots_to_buy: HashMap<Mineral, usize> = HashMap::new();

    //     'search_robots_to_buy: loop {
    //         let can_buy_robots: Vec<Mineral> = MINERALS.into_iter().filter_map(|m| {
    //             if self.can_buy_robot(&m) {
    //                 Some(m)
    //             } else {
    //                 None
    //             }
    //         }).collect();     

    //         println!("Can buy robots: {:?}", can_buy_robots);
    //         if can_buy_robots.len() == 0 {
    //             break 'search_robots_to_buy
    //         }

    //         let best_robot_we_could_buy = self.get_best_robot_we_could_buy_with_current_income();
    //         println!("Best we could buy: {:?}", best_robot_we_could_buy);

    //         if can_buy_robots.contains(&best_robot_we_could_buy) {
    //             println!("Buying best");
    //             *robots_to_buy.entry(best_robot_we_could_buy).or_default() += 1;
    //             self.pay_for_robot(&best_robot_we_could_buy);
    //             continue 'search_robots_to_buy
    //         }

    //         let mut robots_to_buy_score: HashMap<Mineral, usize> = HashMap::new();

    //         // Can't buy best robot yet.. Do we wait to buy it or we buy a lesser one
    //         robots_to_buy_score.insert(best_robot_we_could_buy, self.robot_score(best_robot_we_could_buy));

    //         for robot in can_buy_robots {
    //             let mut factory_clone = self.clone();
    //             factory_clone.pass_minute(Some(robot));
    //             factory_clone.mine_resources();
    //             let new_robots = HashMap::from([(robot, 1)]);
    //             factory_clone.add_robots(new_robots);
    //             factory_clone.time_remaining -= 1;
    //             robots_to_buy_score.insert(best_robot_we_could_buy, factory_clone.robot_score(best_robot_we_could_buy));
    //         }
    //         println!("Robots to buy score: {:?}", robots_to_buy_score);

    //         let (best_to_buy, _score) = robots_to_buy_score.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    //         println!("Best to buy: {:?}", best_to_buy);

    //         if *best_to_buy == best_robot_we_could_buy {
    //             // We wait with the purchase
    //             break 'search_robots_to_buy;
    //         }

    //         *robots_to_buy.entry(*best_to_buy).or_default() += 1;
    //         self.pay_for_robot(&best_to_buy);                                
    //     }
        
    //     println!("Robots to buy: {:?}", robots_to_buy);
    //     robots_to_buy
    // }

    fn get_best_robot_we_could_buy_with_current_income(&self) -> Mineral {
        for mineral in MINERALS.iter().rev() {            
            if self.blueprint[mineral].iter().all(|(rt, _rp)| {
                self.robots[rt] > 0
            }) {
                return *mineral;
            }
        }
        Mineral::Ore
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
            println!("Collected {} {:?}", count, robot_type);
            *self.minerals.get_mut(robot_type).unwrap() += count;
        }
    }

    fn get_time_and_sequence_till_we_can_buy(&self, mineral: &Mineral) -> (usize, usize) {
        let prices = &self.blueprint[mineral];
        let (mineral, time_till_we_can_buy) = prices.iter().map(|(m, p)| {
            let remaining_price: isize = *p as isize - self.minerals[m] as isize;
            let minutes_to_buy = (remaining_price + self.robots[m] as isize - 1) / self.robots[m] as isize; // Div ceil            
            (m, minutes_to_buy)
        }).max_by(|a, b| a.1.cmp(&b.1)).unwrap();

        let sequence_we_can_buy = (prices[mineral] + self.robots[mineral] - 1) / self.robots[mineral];
        (time_till_we_can_buy as usize, sequence_we_can_buy)
    }

    // fn robot_score(&self, mineral: Mineral) -> usize {
    //     let (time, sequence) = self.get_time_and_sequence_till_we_can_buy(&mineral);
    //     println!("m: {:?}, t: {}, s: {}, tr: {}", mineral, time, sequence, self.time_remaining);
    //     if time > self.time_remaining {
    //         return 0;
    //     }
    //     (self.time_remaining - time) / sequence + 1
    //     // let number_of_robots = (self.time_remaining + time / sequence) + 1;
    //     // number_of_robots
    // }

    fn time_and_sequence_till_buy(&self, mineral: &Mineral) -> (usize, usize) {        
        let prices = &self.blueprint[mineral];
        let (_, time_and_sequence) = prices.iter().map(|(m, p)| {
            let remaining_price: isize = *p as isize - self.minerals[m] as isize;
            let minutes_to_buy = if self.robots[m] == 0 {
                let i = MINERALS.iter().position(|m| m == mineral).unwrap();
                let prev = self.time_and_sequence_till_buy(&MINERALS[i - 1]);
                (prev.0 + remaining_price as usize, prev.1 + p)
            } else {
                (
                    ((remaining_price + self.robots[m] as isize - 1) / self.robots[m] as isize) as usize,
                    ((*p as isize + self.robots[m] as isize - 1) / self.robots[m] as isize) as usize,
                ) // Div ceil            
            };
            // let minutes_to_buy = (remaining_price + self.robots[m] as isize - 1) / self.robots[m] as isize; // Div ceil            
            (m, minutes_to_buy)
        }).max_by(|a, b| a.1.cmp(&b.1)).unwrap();        
        time_and_sequence
    }

    fn num_can_buy(&self, mineral: &Mineral) -> usize {
        let (time, sequence) = self.time_and_sequence_till_buy(mineral);
        println!("t: {}, s: {}, m: {:?}", time, sequence, mineral);
        (self.time_remaining + time) / sequence + 1
    }
}