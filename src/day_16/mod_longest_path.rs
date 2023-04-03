// My implementation of this monster of a problem
// Code works, but for part 2 takes too long with real input 
// (for example it works fine).

use std::{collections::{HashMap, VecDeque}, io};
use std::cmp::Ordering;
use anyhow::{Error, Context};
use itertools::Itertools;
use std::str::FromStr;

// Guesses
// 1786 -- too low

pub struct ProboscideaVolcanium {
    map: HashMap<String, Valve>
}

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: i32,
    leads_to: Vec<String>
}


impl FromStr for Valve {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once("; ").unwrap();
        let lhs = lhs.strip_prefix("Valve ").context("Invalid valve string")?;
        let (name, rest) = lhs.split_at(2);
        let val = rest.strip_prefix(" has flow rate=").context("Invalid valve string")?.parse().context("Can't parse value to integer")?;
        let leads_to = match rhs.strip_prefix("tunnels lead to valves ") {
            Some(rhs) => rhs.split(", ").map(|s| s.to_owned()).collect(),
            None => {
                let single_valve = rhs.strip_prefix("tunnel leads to valve ").unwrap();
                vec![single_valve.to_owned()]
            }
        };
        Ok(Valve {
            name: name.to_string(),
            flow_rate: val,
            leads_to
        })
    }
}

impl crate::Advent for ProboscideaVolcanium {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let map: HashMap<String, Valve> = data.lines().map(|l| {
            let valve = Valve::from_str(l).unwrap();
            
            (valve.name.clone(), valve)
        }).collect();

        ProboscideaVolcanium { map }
    }

    fn part_01(&self) -> String {  
        let (max_pressure, came_from) = self.get_max_pressure(30);
        println!("Max cost: {}\nCame from: {:?}", max_pressure, came_from);
        max_pressure.to_string()
    }

    fn part_02(&self) -> String {
        let (pressure, came_from) = self.get_max_pressure_double(26);
        // let (second_pressure, second_came_from) = self.get_max_pressure(26);
        println!("Second pressure: {}\nCame from: {:?}", pressure, came_from);
        2.to_string()
    }
}


// fn cmp_cost(this: &(i32, i32), that: &(i32, i32)) -> Ordering {
//     // Compares two vertex values. Returns Ordering::Greater if this is better than that etc.

//     // Normal check (higher is better)
//     let pressure_release_cmp = this.0.cmp(&that.0);
//     if pressure_release_cmp.is_eq() {
//         // Inverse check (lower is better)
//         return this.1.cmp(&that.1).reverse();
//     }
//     pressure_release_cmp
// }

// fn construct_path(end_node: String, came_from: &HashMap<String, Option<String>>) -> Vec<String> {
//     let mut current = &end_node;
//     let mut path = vec![];
//     while let Some(prev) = &came_from[current] {
//         println!("{} -> {}", current, prev);
//         path.push(prev.clone());
//         if prev == "AA" {
//             return path;
//         }
//         current = prev;
//     }
//     path
// }

fn sort_pair(lhs: &String, rhs: &String) -> (String, String) {
    match lhs.cmp(&rhs) {
        Ordering::Greater => (rhs.clone(), lhs.clone()),
        _ => (lhs.clone(), rhs.clone())
    }
}

impl ProboscideaVolcanium {
    fn get_max_pressure(&self, time_limit: i32) -> (i32, Vec<String>) {
        // cost = (sum_pressure, time_passed)
        let mut queue: VecDeque<(String, (i32, i32), Vec<String>)> = VecDeque::new();
        let current: String = "AA".to_owned();
        
        let visited: Vec<String> = vec![];
        queue.push_back((current.clone(), (0, 0), visited));
        let mut node_cost_map: HashMap<String, (i32, i32)> = HashMap::new();
        let mut distances_map: HashMap<String, HashMap<String, i32>> = HashMap::new();
        let vertices: Vec<String> = self.map.keys().sorted().cloned().collect();        
        for vertex in vertices.iter() {
            node_cost_map.insert(vertex.clone(), (i32::MIN, i32::MAX));
            distances_map.insert(vertex.clone(), self.get_distances(&vertex));
        }
        
        let mut max_pressure = i32::MIN;
        let mut came_from: Vec<String> = vec![];
        // let mut iteration = 0;
        while let Some((u, cost, visited)) =  queue.pop_front() {                                    
            // println!("{} {:?} -> {:?}, Cost: {:?}, Visited: {:?}", iteration, prev, u, cost, visited);            
            // if (iteration + 1) % 10 == 0 {                                
            //     wait_user_input();
            // }

            if cost.0 > max_pressure {
                max_pressure = cost.0;
                came_from = visited.clone();
            }
            let distance_map = &distances_map[&u];
            for vertex in vertices.iter() {
                if !visited.contains(&vertex) {
                    let distance = distance_map[vertex];
                    let (pressure_release, time_to_release_pressure) = self.get_pressure_and_time(vertex, distance, cost.1, time_limit);

                    if time_to_release_pressure > time_limit {
                        continue;
                    }

                    let mut new_visited = visited.clone();
                    new_visited.push(vertex.clone());

                    let new_tp = time_to_release_pressure;
                    let new_pr = cost.0 + pressure_release;
                    let new_cost = (new_pr, new_tp);

                    let current_cost = node_cost_map[vertex];
                    // if cmp_cost(&new_cost, &current_cost).is_ge() {
                    if new_cost.0 + pressure_release >= current_cost.0 {
                        *node_cost_map.get_mut(vertex).unwrap() = new_cost;
                        queue.push_back((vertex.clone(), new_cost, new_visited.clone()));                        
                    }
                }
            }
            // iteration += 1;    
        }
        return (max_pressure, came_from)    
    }         

    fn get_max_pressure_double(&self, time_limit: i32) -> (i32, Vec<String>) {
        // cost = (sum_pressure, time_passed)
        let mut queue: VecDeque<((String, (i32, i32)), (String, (i32, i32)), Vec<String>)> = VecDeque::new();
        let current: String = "AA".to_owned();
        
        let visited: Vec<String> = vec![];
        queue.push_back((
            (current.clone(), (0, 0)),
            (current.clone(), (0, 0)),
            visited
        ));
        let mut node_cost_map: HashMap<(String, String), (i32, i32)> = HashMap::new();
        let mut distances_map: HashMap<String, HashMap<String, i32>> = HashMap::new();
        let vertices: Vec<String> = self.map.keys().sorted().cloned().collect();
        println!("Vertices: {:?}", vertices);
        for vertex in vertices.iter() {            
            distances_map.insert(vertex.clone(), self.get_distances(&vertex));
        }

        let vertices_perm: Vec<_> = vertices.iter().permutations(2).map(|vs| {
            (vs[0].clone(), vs[1].clone())
            // sort_pair(vs[0], vs[1])
        }).filter(|(a, b)| a != b).collect();
        println!("Perms: {:?}", vertices_perm);
        println!("Perms: {:?}", vertices_perm.len());
        

        for pair in vertices_perm.iter() {
            node_cost_map.insert(pair.clone(), (i32::MIN, i32::MAX));
        }
        println!("Node cost map len: {:?}", node_cost_map.len());

        
        let mut max_pressure = i32::MIN;
        let mut came_from: Vec<String> = vec![];
        
        let mut iteration = 0;
        while let Some(((u, u_cost), (i, i_cost), visited)) =  queue.pop_back() {                                                
            if (iteration + 1) % 10000 == 0 {    
                println!("{} {:?}({:?}) - {:?}({:?}), Visited: {:?}, queue len: {}", iteration, u, u_cost, i, i_cost, visited, queue.len());            
                // wait_user_input();
            }

            let pressure_sum = u_cost.0 + i_cost.0;
            if pressure_sum > max_pressure {
                max_pressure = pressure_sum;
                came_from = visited.clone();
            }
            let u_distance_map = &distances_map[&u];
            let i_distance_map = &distances_map[&i];
            for (u_vertex, i_vertex) in vertices_perm.iter() {
                if !visited.contains(u_vertex) && !visited.contains(i_vertex) {
                    if u_vertex == i_vertex {
                        panic!("Same node! :{}, {}", u_vertex, i_vertex);
                    }
                    let u_distance = u_distance_map[u_vertex];
                    let u_pt = self.get_pressure_and_time(u_vertex, u_distance, u_cost.1, time_limit);
                    
                    let i_distance = i_distance_map[i_vertex];
                    let i_pt = self.get_pressure_and_time(i_vertex, i_distance, i_cost.1, time_limit);

                    if u_pt.1 > time_limit || i_pt.1 > time_limit {
                        continue;
                    }
                    let mut new_visited = visited.clone();
                    new_visited.push(u_vertex.clone());
                    new_visited.push(i_vertex.clone());

                    let u_new_cost = (u_cost.0 + u_pt.0, u_pt.1);
                    let i_new_cost = (i_cost.0 + i_pt.0, i_pt.1);
                    
                    // let pair = sort_pair(u_vertex, i_vertex);
                    let pair = (u_vertex.clone(), i_vertex.clone());

                    let current_cost = node_cost_map.get_mut(&pair).unwrap();
                    if (u_new_cost.0 + u_pt.0 + i_new_cost.0 + i_pt.0) >= current_cost.0 {
                        *current_cost = (u_new_cost.0 + i_new_cost.0, std::cmp::max(u_new_cost.1, i_new_cost.1));
                        queue.push_back((
                            (u_vertex.clone(), u_new_cost),
                            (i_vertex.clone(), i_new_cost),
                            new_visited
                        ));
                    }
                }
            }
            // for u_vertex in vertices.iter() {
            //     if !visited.contains(&u_vertex) {                
            //         for i_vertex in vertices.iter() {
            //             if i_vertex != u_vertex && !visited.contains(&i_vertex) {                        
            //                 let u_distance = u_distance_map[u_vertex];
            //                 let u_pt = self.get_pressure_and_time(u_vertex, u_distance, u_cost.1, time_limit);
                            
            //                 let i_distance = i_distance_map[i_vertex];
            //                 let i_pt = self.get_pressure_and_time(i_vertex, i_distance, i_cost.1, time_limit);

            //                 if u_pt.1 > time_limit || i_pt.1 > time_limit {
            //                     continue;
            //                 }
            //                 let mut new_visited = visited.clone();
            //                 new_visited.push(u_vertex.clone());
            //                 new_visited.push(i_vertex.clone());

            //                 let u_new_cost = (u_cost.0 + u_pt.0, u_pt.1);
            //                 let i_new_cost = (i_cost.0 + i_pt.0, i_pt.1);
                            
            //                 let pair = sort_pair(u_vertex, i_vertex);

            //                 let current_cost = node_cost_map.get_mut(&pair).unwrap();
            //                 if (u_new_cost.0 + u_pt.0 + i_new_cost.0 + i_pt.0) >= current_cost.0 {
            //                     *current_cost = (u_new_cost.0 + i_new_cost.0, std::cmp::max(u_new_cost.1, i_new_cost.1));
            //                     queue.push_back((
            //                         (u_vertex.clone(), u_new_cost),
            //                         (i_vertex.clone(), i_new_cost),
            //                         new_visited
            //                     ));
            //                 }
            //             }
            //         }
            //     }
            // }
            iteration += 1;    
        }

        
        return (max_pressure, came_from)    
    }        

    fn get_pressure_and_time(&self, node: &String, distance: i32, time_passed: i32, time_limit: i32) -> (i32, i32) {
        let time_to_release_pressure = time_passed + distance + 1;
        let pressure_release = (time_limit - time_to_release_pressure) * self.map[node].flow_rate;
        return (pressure_release, time_to_release_pressure);
    }


    // fn get_pressure_map(&self, distance_map: &HashMap<String, i32>, time_passed: i32) -> HashMap<String, (i32, i32)> {
    //     let mut pressure_map: HashMap<_, _> = HashMap::new();
    //     for (key, distance) in distance_map.iter() {
    //         let (pressure_release, time_to_release_pressure) = self.get_pressure_and_time(key, *distance, time_passed);
    //         if time_to_release_pressure <= 30 {
    //             pressure_map.insert(key.clone(), (pressure_release, time_to_release_pressure));
    //         }
    //     }
    //     pressure_map
    // }

    fn get_distances(&self, start: &String) -> HashMap<String, i32> {
        let mut queue: Vec<String> = self.map.keys().map(|k| k.clone()).collect();
        let mut prev_map: HashMap<String, Option<String>> = HashMap::new();
        let mut distance_map: HashMap<String, i32> = HashMap::new();
        let mut current_valve = start;

        for key in queue.iter() {
            distance_map.insert(key.clone(), i32::MAX);
            prev_map.insert(key.clone(), None);
        }
        *distance_map.get_mut(current_valve).unwrap() = 0;

        while queue.len() > 0 {
            let (name, distance, pos) = distance_map
                .iter()
                .filter_map(|(k, v)| {
                    if let Some(pos) = queue.iter().position(|v| v == k) {
                        Some((k.clone(), v.clone(), pos))
                    } else {
                        None
                    }
                })
                .min_by(|a, b| a.1.cmp(&b.1)).unwrap();

            queue.remove(pos);

            current_valve = &name;
            let neighbours: Vec<_> = self.map[current_valve].leads_to.iter().filter(|n| {
                queue.contains(n)
            }).collect();

            for neighbour in neighbours.into_iter() {
                let alt_distance = distance + 1;
                if alt_distance < distance_map[neighbour] {
                    *distance_map.get_mut(neighbour).unwrap() = alt_distance;
                    *prev_map.get_mut(neighbour).unwrap() = Some(name.clone());
                }
            }
        }
        distance_map
    }    
}
