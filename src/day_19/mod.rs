use std::str::FromStr;
use anyhow::{Result, Error, anyhow};
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
        println!("Passed minute: {}", self.time_limit - self.time_remaining);

        let bought_robots = self.buy_robots();

        self.mine_resources();

        self.add_robots(bought_robots);

        self.time_remaining -= 1;
    }

    fn buy_robots(&mut self) -> HashMap<Mineral, usize> {
        let mut robots_to_buy: HashMap<Mineral, usize> = HashMap::new();

        'search_robots_to_buy: loop {
            let can_buy_robots: Vec<Mineral> = MINERALS.into_iter().filter_map(|m| {
                if self.can_buy_robot(&m) {
                    Some(m)
                } else {
                    None
                }
            }).collect();     

            println!("Can buy robots: {:?}", can_buy_robots);
            if can_buy_robots.len() == 0 {
                break 'search_robots_to_buy
            }

            let best_robot_we_could_buy = self.get_best_robot_we_could_buy_with_current_income();
            println!("Best we could buy: {:?}", best_robot_we_could_buy);

            if can_buy_robots.contains(&best_robot_we_could_buy) {
                println!("Buying best");
                *robots_to_buy.entry(best_robot_we_could_buy).or_default() += 1;
                self.pay_for_robot(&best_robot_we_could_buy);
                continue 'search_robots_to_buy
            }

            let mut robots_to_buy_score: HashMap<Mineral, usize> = HashMap::new();

            // Can't buy best robot yet.. Do we wait to buy it or we buy a lesser one
            let (time, sequence) = self.get_time_and_sequence_till_we_can_buy(&best_robot_we_could_buy);
            let number_of_robots = (self.time_remaining - time / sequence) + 1;
            robots_to_buy_score.insert(best_robot_we_could_buy, number_of_robots);

            for robot in can_buy_robots {
                let mut factory_clone = self.clone();
                factory_clone.mine_resources();
                let new_robots = HashMap::from([(robot, 1)]);
                factory_clone.add_robots(new_robots);
                factory_clone.time_remaining -= 1;

                let (time, sequence) = factory_clone.get_time_and_sequence_till_we_can_buy(&best_robot_we_could_buy);
                let number_of_robots = (factory_clone.time_remaining  - time / sequence) + 1;
                robots_to_buy_score.insert(robot, number_of_robots);
            }
            println!("Robots to buy score: {:?}", robots_to_buy_score);

            let (best_to_buy, _score) = robots_to_buy_score.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

            println!("Best to buy: {:?}", best_to_buy);

            if *best_to_buy == best_robot_we_could_buy {
                // We wait with the purchase
                break 'search_robots_to_buy;
            }

            *robots_to_buy.entry(*best_to_buy).or_default() += 1;
            self.pay_for_robot(&best_to_buy);                                
        }
        
        println!("Robots to buy: {:?}", robots_to_buy);
        robots_to_buy
    }

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
}