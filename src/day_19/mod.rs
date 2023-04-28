use std::str::FromStr;
use std::time;
use anyhow::{Result, Error, anyhow};
use owo_colors::OwoColorize;
use std::collections::{HashMap, VecDeque};
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
        // 1.to_string()
        let mut quality_levels_sum = 0;
        for (i, blueprint) in self.blueprints.iter().enumerate() {
            println!("Searching blueprint: {}", i);
            let factory = Factory::new(blueprint.clone(), 24);              
            let max_geodes = factory.find_best_strategy();
            let quality_level = (i + 1) * max_geodes;
            println!("{} ({}) -> {}", i, max_geodes, quality_level);
            quality_levels_sum += quality_level;
        }
        
        quality_levels_sum.to_string()
    }

    fn part_02(&self) -> String {
        // let factory = Factory::new(self.blueprints[0].clone(), 32);
        // let result = factory.find_best_strategy();
        // result.to_string()
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

    // fn can_buy(&self, other_states: &HashMap<Mineral, MineralState>) -> bool {
    //     // Can we buy the robot now
    //     self.prices.iter().all(|(m, price)| {
    //         other_states[m].minerals >= *price
    //     })
    // }

    fn could_buy(&self, other_states: &HashMap<Mineral, MineralState>) -> bool {
        self.prices.iter().all(|(m, _)| {
            other_states[m].robots > 0
        })
    }

    fn add_robots(&mut self, count: usize) {
        self.robots += count;
    }

    fn mine(&mut self) {
        self.minerals += self.robots;
    }

    fn buy_per_minute(&self) -> f32 {
        if self.robots == 0 {
            return 0f32;
        }
        1f32 / self.robots as f32
    }
}

#[derive(Debug, PartialEq)]
struct Score {
    nr_geodes: usize,
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
    time_passed: usize,
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
            time_passed: 0,
        }
    }

    fn time_remaining(&self) -> usize {
        self.time_limit - self.time_passed
    }


    fn state(&self, mineral: &Mineral) -> &MineralState {
        &self.mineral_states[mineral]
    }

    fn state_mut(&mut self, mineral: &Mineral) -> &mut MineralState {
        self.mineral_states.get_mut(mineral).unwrap()
    }

    fn mineral_buy_per_minute(&self, mineral: &Mineral) -> f32 {
        let state = self.state(&mineral);
        let buy_per_min = state.buy_per_minute();
        if buy_per_min == 0f32 {
            let min_buy_per_min = state.prices.iter().map(|(m, p)| {
                let prev_state = self.state(&m);
                let prev_per_min = if prev_state.robots == 0 {
                    self.mineral_buy_per_minute(&m)
                } else {
                    self.mineral_buy_per_minute(&m) / *p as f32
                };

                (m, prev_per_min)
            }).min_by(|a, b| {
                a.1.partial_cmp(&b.1).unwrap()
            }).unwrap();

            min_buy_per_min.1
        } else {
            buy_per_min
        }
    }

    fn mineral_minutes_to_buy(&self, mineral: &Mineral) -> usize {
        // In how many minutes we can buy a robot of mineral type taking into account 
        // the current resources we have.

        let state = self.state(&mineral);              
        let max_minutes_to_buy = state.prices.iter().map(|(m, p)| {
            let prev_state = self.state(&m);
            if prev_state.robots == 0 {
                self.mineral_minutes_to_buy(m) + 1 + *p
            } else {
                if prev_state.minerals >= *p {
                    return 0
                }
                let remainder = *p - prev_state.minerals;
                (remainder + prev_state.robots - 1) / prev_state.robots
            }
        }).max().unwrap();
        max_minutes_to_buy
    }

    fn sequence_to_buy(&self, mineral: &Mineral) -> usize {
        // In how many minutes we can buy a robot of mineral type without taking 
        // into account the current resources we have.
        let state = self.state(&mineral);              
        let max_minutes_to_buy = state.prices.iter().map(|(m, p)| {
            let prev_state = self.state(&m);
            if prev_state.robots == 0 {
                self.sequence_to_buy(m) + 1 + *p
            } else {
                (*p + prev_state.robots - 1) / prev_state.robots
            }
        }).max().unwrap();
        max_minutes_to_buy
        

        // let max_sequence_to_buy = state.prices.iter().map(|(m, p)| {
        //     let mut sequence_to_buy = self.state(m).sequence_to_buy(*p);
        //     if sequence_to_buy == usize::MAX {
        //         sequence_to_buy = self.sequence_to_buy(m) + 1 + *p;
        //     }
        //     (m, sequence_to_buy)
        // }).max_by(|a, b| {
        //     a.1.cmp(&b.1)
        // }).unwrap();

        // max_sequence_to_buy.1
    }
    
    fn predicted_score(&self) -> usize {
        let minutes_to_buy = self.mineral_minutes_to_buy(&Mineral::Geode);
        let sequence_to_buy = self.sequence_to_buy(&Mineral::Geode);
        if minutes_to_buy > self.time_remaining() {
            return 0;
        }
        let remaining_time = self.time_remaining() - minutes_to_buy;

        let state = self.state(&Mineral::Geode);
        let mut total_mined = 
            state.minerals +
            state.robots * self.time_remaining() +
            (self.time_remaining() - minutes_to_buy);
        for x in 0..(remaining_time / sequence_to_buy) {
            let res = remaining_time - (sequence_to_buy * (x + 1));
            total_mined += res;
        }
        total_mined
        // result + (remaining_time / sequence_to_buy)
    }

    fn find_best_strategy(&self) -> usize {
        let mut time_passed = self.time_passed;
        // Mineral to buy, time to buy, Factory
        let mut factories: Vec<(Mineral, usize, Factory)> = Vec::new();

        for mineral in MINERALS.iter().filter(|m| {
            self.state(m).could_buy(&self.mineral_states)
        }) {            
            let minutes_till_buy = self.mineral_minutes_to_buy(&mineral);
            let buy_at = self.time_passed + minutes_till_buy;
            factories.push((*mineral, buy_at, self.clone()));            
        }

        while time_passed < self.time_limit {
            // println!("Nr. Factories: {}, Time passed: {}", factories.len(), time_passed + 1);

            let mut bought_for_factory_indexes: Vec<usize> = vec![];
            for (i, (m, ttb, f)) in factories.iter_mut().enumerate() {
                // println!("I:{}, M: {:?}, Ttb: {}, f: {}", i, m, ttb, f);
                if *ttb == f.time_passed {
                    bought_for_factory_indexes.push(i);
                    f.pass_minute(vec![m.clone()]);
                } else {
                    f.pass_minute(vec![]);
                }
            }
            time_passed += 1;
            for (_, _, f) in factories.iter() {
                if f.time_passed != time_passed {
                    panic!("Time passed doesn't match!");
                }
            }

            let mut best_score = usize::MIN;
            for (_, _, f) in factories.iter() {
                let score = f.predicted_score();
                if score > best_score {
                    best_score = score;
                }
            }

            // if time_passed <= 5 || time_passed == 18 {
            //     for (m, _ttb, f) in factories.iter() {
            //         let score = f.predicted_score();
            //         println!("{:?} score {}", m, score);
            //     }                                
            // }

            // println!("Best score: {}", best_score);

            let mut remove_indexes: Vec<usize> = vec![];
            for (i, (m, ttb, f)) in factories.iter().enumerate().rev() {
                let score = f.predicted_score();
                if time_passed - 1 == *ttb && score < best_score {
                    // println!("Removing: {:?} with score: {}", m, score);
                    remove_indexes.push(i)
                }
            }
                
            for i in remove_indexes {
                factories.remove(i);
            }


            let mut new_factories: Vec<(Mineral, usize, Factory)> = vec![];
            let mut remove_indexes: Vec<usize> = vec![];
            for (i, (m, ttb, f)) in factories.iter().enumerate().rev() {                
                if time_passed - 1 == *ttb {
                    // println!("Bought {:?}.", m);
                    remove_indexes.push(i);
                    for mineral in MINERALS.iter().filter(|m| {
                        f.state(&m).could_buy(&f.mineral_states)                        
                    }) {
                        let minutes_till_buy = f.mineral_minutes_to_buy(&mineral);
                        let buy_at = f.time_passed + minutes_till_buy;
                        new_factories.push((*mineral, buy_at, f.clone()));
                    }            
                }
            }
            // println!("Remove indexes: {:?}", remove_indexes);            

            for i in remove_indexes {
                factories.remove(i);
            }  

            // println!("New factories: ");
            // for (m, ttb, f) in new_factories.iter() {
            //     println!("M: {:?}, TTB: {}", m, ttb);
            //     println!("{}", f);
            // }

            factories.extend(new_factories);
            // println!("T:{}", time_passed);
            // wait_user_input();            
            // println!("======================");
        }

        // println!("Best strategies: ");
        // println!("Factories: ");
        // for (m, ttb, f) in factories.iter() {
            // println!("M: {:?}, TTB: {}", m, ttb);
            // println!("{}", f);
        // }

        let max_geodes = factories.iter().map(|(_, _, f)| {
            f.state(&Mineral::Geode).minerals
        }).max().unwrap();

        max_geodes
    }

    fn pass_minute(&mut self, buy_robots: Vec<Mineral>) {
        for robot in &buy_robots {
            let prices = self.state(robot).prices.clone();
            for (m, price) in prices {
                // println!("M: {:?}, Minerals: {}, Price: {}", m, self.state(&m).minerals, price);
                self.state_mut(&m).minerals -= price;
            }
        }

        self.mine_resources();

        for robot in &buy_robots {
            self.state_mut(robot).add_robots(1);
        }
        
        self.time_passed += 1;
    }

    fn mine_resources(&mut self) {
        for state in self.mineral_states.values_mut() {
            state.mine();
        }
    }
}

impl std::fmt::Display for Factory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "----------{}-------------", self.index)?;
        writeln!(f, "Passed minute: {}, State:", self.time_passed)?;
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
    fn test_first_case() {
        let b = &*BLUEPRINT;
        let mut factory = Factory::new(b.clone(), 24);
        use Mineral::*;
        let solution: HashMap<usize, Mineral> = HashMap::from([            
            (3, Clay),
            (5, Clay),
            (7, Clay),
            (11, Obsidian),
            (12, Clay),
            (15, Obsidian),
            (18, Geode),
            (21, Geode)
        ]);

        let solution_time_remaining: Vec<usize> = vec![
            25, 24, 23, 22, 21, 
            14, 13, 11, 10, 9,
            8, 7, 6, 5, 4,
            2, 1, 0, 2, 1,
            0, 3, 2, 1
        ];

        let solution_seq: Vec<usize> = vec![
            25, 25, 25, 22, 22, 
            15, 15, 13, 13, 13,
            13, 7, 7, 7, 7,
            4, 4, 4, 4, 4,
            4, 4, 4, 4
        ];
        
        let solution_score: Vec<usize> = vec![
            0, 0, 0, 0, 0, 
            5, 5, 6, 6, 6,
            6, 6, 6, 6, 6,
            10, 10, 10, 10, 10,
            10, 9, 9, 9
        ];

        while factory.time_passed < factory.time_limit {
            assert_eq!(factory.mineral_minutes_to_buy(&Geode), solution_time_remaining[factory.time_passed]);
            assert_eq!(factory.sequence_to_buy(&Geode), solution_seq[factory.time_passed]);
            assert_eq!(factory.predicted_score(), solution_score[factory.time_passed]);
            if solution.contains_key(&(factory.time_passed + 1)) {
                factory.pass_minute(vec![solution[&(factory.time_passed + 1)]]);
            } else {
                factory.pass_minute(vec![]);
            }            
        }
    }

}

