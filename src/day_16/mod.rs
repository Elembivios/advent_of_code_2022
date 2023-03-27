
// Copied code from: https://github.com/Crazytieguy/advent-of-code/blob/master/2022/src/bin/day16/main.rs
// My solution doesn't run this fast xD

use std::{collections::HashMap, cmp::Reverse};

use itertools::Itertools;

type Valve<'a> = (&'a str, u8, Vec<&'a str>);
type FlowRates = Vec<u8>;
type FlowRateIndices = Vec<usize>;
type ShortesPathLenghts = Vec<Vec<u8>>;

#[derive(Debug)]
pub struct ProboscideaVolcanium {
    // valves: Vec<Valve<'a>>,
    flow_rates: FlowRates,
    shortest_path_lengths: ShortesPathLenghts,
    flow_rate_indices: FlowRateIndices,
    starting_node: usize
}

impl crate::Advent for ProboscideaVolcanium {
    fn new(data: &str) -> Self {
        let valves: Vec<Valve> = data.lines().map(|l| {
            let (lhs, rhs) = l.split_once("; ").unwrap();
            let lhs = lhs.strip_prefix("Valve ").unwrap();
            let (name, rest) = lhs.split_at(2);
            let flow_rate = rest.strip_prefix(" has flow rate=").unwrap().parse().unwrap();
            let tunnels: Vec<_> = rhs
                .strip_prefix("tunnels lead to valves ")
                .or(rhs.strip_prefix("tunnel leads to valve "))
                .unwrap().split(", ").collect();
            (name, flow_rate, tunnels)

        }).collect();

        let shortest_path_lengths_uncompressed = floyd_warshall(&valves);

        let interesting_valve_indices: Vec<_> = valves
            .iter()
            .enumerate()
            .filter(|(_, v)| v.0 == "AA" || v.1 > 0)
            .map(|(i, _)| i)
            .collect();

        let flow_rates: Vec<_> = interesting_valve_indices
            .iter()
            .map(|i| valves[*i].1)
            .collect();

        let shortest_path_lengths: Vec<Vec<_>> = interesting_valve_indices
            .iter()
            .map(|&i| {
                interesting_valve_indices
                    .iter()
                    .map(|&j| {
                        shortest_path_lengths_uncompressed[i][j]
                    }).collect()
            }).collect();

        let starting_node = interesting_valve_indices
            .iter()
            .position(|&i| valves[i].0 == "AA")
            .unwrap();

        let sorted_flow_rate_indices: Vec<_> = flow_rates
            .iter()
            .enumerate()
            .sorted_unstable_by_key(|&(_, &flow)| Reverse(flow))
            .map(|(i, _)| i)
            .collect();
                
        Self { 
            flow_rates, 
            shortest_path_lengths, 
            flow_rate_indices: sorted_flow_rate_indices, 
            starting_node
        }
    }

    fn part_01(&self) -> String {
        let mut best = 0;
        branch_and_bound(
            &self.flow_rates, 
            &self.flow_rate_indices, 
            &self.shortest_path_lengths, 
            State::new(self.starting_node as u8, 30),
            &mut [], 
            &mut best, 
            |bound, best| bound > best
        );
        best.to_string()
    }

    fn part_02(&self) -> String {
        let mut best_per_visited = vec![0; u16::MAX as usize];
        branch_and_bound(
            &self.flow_rates, 
            &self.flow_rate_indices, 
            &self.shortest_path_lengths, 
            State::new(self.starting_node as u8, 26),
            &mut best_per_visited, 
            &mut 0,
            // This could techically produce an incorrect result, 
            // but it doesn't on my input
            |bound, best| bound > best * 3 / 4
        );

        let best_per_visited_filtered_sorted: Vec<_> = best_per_visited
            .into_iter()
            .enumerate()
            .filter(|&(_, best)| best > 0)
            .map(|(i, best)| (i as u16, best))
            .sorted_unstable_by_key(|&(_, best)| Reverse(best))
            .collect();

        let mut best = 0;

        for (i, &(my_visited, my_best)) in best_per_visited_filtered_sorted.iter().enumerate() {
            for &(elephant_visited, elephant_best) in &best_per_visited_filtered_sorted[i + 1..] {
                let score = my_best + elephant_best;
                if score <= best {
                    break;
                }
                if my_visited & elephant_visited == 0 {
                    best = score;
                    break;
                }
            }
        }

        best.to_string()
    }
}


fn floyd_warshall(valves: &Vec<Valve>) -> Vec<Vec<u8>> {
    // Returns shortest paths between all nodes.
    let valve_name_to_ids: HashMap<&str, _> = valves 
        .iter()
        .enumerate()
        .map(|(i, &(name, _, _))| (name, i))
        .collect();

    let mut dist = vec![vec![u8::MAX; valves.len()]; valves.len()];

    for (i, (_, _, tunnels)) in valves.iter().enumerate() {
        for tunnel in tunnels {
            let j = valve_name_to_ids[tunnel];
            dist[i][j] = 1;
        }
    }

    (0..dist.len()).for_each(|i| {
        dist[i][i] = 0;
    });

    for k in 0..dist.len() {
        for i in 0..dist.len() {
            for j in 0..dist.len() {
                let (result, overflow) = dist[i][k].overflowing_add(dist[k][j]);
                if !overflow && dist[i][j] > result {
                    dist[i][j] = result;
                }
            }
        }
    }

    dist
}


fn branch_and_bound(
    flow_rates: &FlowRates,
    sorted_flow_rate_indices: &[usize],
    shortest_path_lengths: &ShortesPathLenghts,
    state: State,
    best_for_visited: &mut [u16],
    best: &mut u16,
    filter_bound: impl Fn(u16, u16) -> bool + Copy
) {
    if let Some(cur_best) = best_for_visited.get_mut(state.visited as usize) {
        *cur_best = state.pressure_released.max(*cur_best);
    }
    *best = state.pressure_released.max(*best);

    let bound_branch_pairs: Vec<_> = state
        .branch(flow_rates, shortest_path_lengths)
        .into_iter()
        .map(|state| (state.bound(flow_rates, sorted_flow_rate_indices), state))
        .filter(|&(bound, _)| filter_bound(bound, *best))
        .sorted_unstable_by_key(|(bound, _)| Reverse(*bound))
        .map(|(_, branch)| branch)
        .collect();

    for branch in bound_branch_pairs {
        branch_and_bound(flow_rates, sorted_flow_rate_indices, shortest_path_lengths, branch, best_for_visited, best, filter_bound);
    }
        
}

#[derive(Default, Debug, Clone, Copy)]
struct State {
    visited: u16,
    avoid: u16,
    pressure_released: u16,
    minutes_remaining: u8,
    position: u8,
}

impl State {
    fn new(position: u8, minutes_remaining: u8) -> Self {
        Self {
            visited: 0,
            avoid: 1 << position,
            pressure_released: 0,
            minutes_remaining,
            position
        }
    }

    fn can_visit(self, i: usize) -> bool {
        (self.visited | self.avoid) & (1 << i) == 0
    }

    // Assuming the shortest path lengths are all 1, the best
    // solution is to visit the valves in order of descending 
    // flow rate.
    fn bound(self, flow_rates: &FlowRates, sorted_flow_rate_indices: &[usize]) -> u16 {
        self.pressure_released + (0..=self.minutes_remaining)
            .rev()
            .step_by(2)
            .skip(1)
            .zip(
                sorted_flow_rate_indices
                    .iter()
                    .filter(|&&i| self.can_visit(i))
                    .map(|&i| flow_rates[i])
            )
            .map(|(minutes, flow)| minutes as u16 * flow as u16)
            .sum::<u16>()
    }

    fn branch<'a> (self, flow_rates: &'a FlowRates, shortest_path_lengths: &'a ShortesPathLenghts) -> impl IntoIterator<Item=Self> + 'a {
        shortest_path_lengths[self.position as usize]
            .iter()
            .enumerate()
            .filter(move |&(destination, _distance)| self.can_visit(destination))
            .filter_map(move |(destination, distance)| {
                let minutes_remaining = self.minutes_remaining.checked_sub(*distance + 1)?;
                Some(State {
                    visited: self.visited | (1 << destination),
                    avoid: self.avoid,
                    pressure_released: self.pressure_released + minutes_remaining as u16 * flow_rates[destination] as u16,
                    minutes_remaining,
                    position: destination as u8
                })
            })
    }
}