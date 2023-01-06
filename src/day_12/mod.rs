use crate::utils::point::{Coord, Point, Grid};
use std::collections::HashMap;
use std::io;
use owo_colors::{OwoColorize};

type P<'a> = Point<usize, &'a u32>;
type C = Coord<usize>;


pub struct HillClimbingAlhorithm {
    grid: Grid<u32>,
    start: C,
    end: C
}

impl crate::Advent for HillClimbingAlhorithm {
    fn new(data: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let map: Vec<Vec<u32>> = data
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.chars().enumerate().map(|(x, c)| {
                    match c {
                        'S' => {
                            start = (x, y);
                            0
                        },
                        'E' => {
                            end = (x, y);
                            25
                        },
                        c => {
                            c.to_digit(36).unwrap() - 10
                        }
                    }
                }).collect()
            }).collect();                
        let grid: Grid<u32> = Grid::new(map);

        HillClimbingAlhorithm {
            grid,
            start: start.into(),
            end: end.into(),
        }
    }

    fn part_01(&self) -> String {
        let heuristic = |current: &C, goals: &Vec<C>| {
            goals.iter().map(|goal| {
                let x = goal.x.abs_diff(current.x);
                let y = goal.y.abs_diff(current.y);
                let future_path = x + y;
                future_path
            }).min().unwrap()            
        };
        let valid_neighbour = |current: &P, neighbour: &P| {
            (*neighbour.value as i32 - *current.value as i32) <= 1
        };

        let path = a_star(
            &self.grid, 
            self.start, 
            vec![self.end], 
            heuristic, 
            valid_neighbour
        );
                
        let path = path.unwrap_or(vec![]);
        let cost = path.iter().count().checked_sub(1).unwrap_or(0);
        cost.to_string()
    }

    fn part_02(&self) -> String {
        let heuristic = |current: &C, goals: &Vec<C>| {
            goals.iter().map(|goal| {
                let x = goal.x.abs_diff(current.x);
                let y = goal.y.abs_diff(current.y);
                let future_path = x + y;
                future_path
            }).min().unwrap()            
        };
        let valid_neighbour = |current: &P, neighbour: &P| {
            (*neighbour.value as i32 - *current.value as i32) >= -1
        };

        let goals: Vec<_> = self.grid.iter_points().filter(|p| *p.value == 0).map(|p| p.coord).collect();
        let path = a_star(
            &self.grid, 
            self.end, 
            goals, 
            heuristic,
            valid_neighbour
        );
        let path = path.unwrap_or(vec![]);
        let cost = path.iter().count().checked_sub(1).unwrap_or(0);
        cost.to_string()
    }
}


fn reconstruct_path(came_from: HashMap<C, C>, current: C) -> Vec<C> {
    let mut total_path: Vec<C> = vec![current.clone()];
    let mut previous_coord = came_from.get(&current);
    while previous_coord.is_some() {
        if let Some(c) = previous_coord {
            total_path.insert(0, c.clone());
            previous_coord = came_from.get(c);
        }
    }
    total_path
}

fn d(_current: &C, _neighbour: &C) -> usize {
    1
}

fn a_star<'a>(grid: &'a Grid<u32>, start: C, goals: Vec<C>, heuristic: fn(&C, &Vec<C>) -> usize, valid_neighbour: fn(&P, &P) -> bool) -> Option<Vec<C>> {
    let mut open_set = vec![start];
    let mut came_from: HashMap<C, C> = HashMap::new();
    let mut g_score: HashMap<C, usize> = grid.iter_coords().map(|c| (c, usize::MAX)).collect();
    g_score.insert(start, 0);
    let mut f_score: HashMap<C, usize> = grid.iter_coords().map(|c| (c, usize::MAX)).collect();
    f_score.insert(start, heuristic(&start, &goals));

    while open_set.len() != 0 {
        let min_f = open_set.iter().map(|c| {
            (c, f_score.get(c).unwrap())
        }).min_by(|a, b| {
            a.1.cmp(b.1)
        }).unwrap();
        let current = *min_f.0;

        if goals.iter().any(|g| *g == current) {
            return Some(reconstruct_path(came_from, current));            
        }

        let pos = open_set.iter().position(|p| *p == current).unwrap();
        open_set.remove(pos);

        for neighbour in grid.neighbour_coords(&current) {
            if !valid_neighbour(&grid.get_point(&current), &grid.get_point(&neighbour)) {
                continue;
            }
            let tentative_g_score = g_score[&current] + d(&current, &neighbour);

            if tentative_g_score < g_score[&neighbour] {
                came_from.insert(neighbour, current);
                let tentative_f_score = tentative_g_score + heuristic(&neighbour, &goals);
                g_score.insert(neighbour, tentative_g_score);
                f_score.insert(neighbour, tentative_f_score);
                if !open_set.contains(&&neighbour) {
                    open_set.push(neighbour);
                }
            }
        }
        // display_and_wait(grid, &open_set, current, &reconstruct_path(&came_from, current));
    }
    None
}

#[allow(dead_code)]
fn display_and_wait(grid: &Grid<u32>, open_set: &Vec<&C>, current: &C, path: &Vec<&C>) {    
    display(grid, open_set, current, path);
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).ok().unwrap(); 
    print!("{esc}c", esc = 27 as char);
}

#[allow(dead_code)]
fn display(grid: &Grid<u32>, open_set: &Vec<&C>, current: &C, path: &Vec<&C>) {    
    for (y, chunk) in grid.map.chunks(grid.width).enumerate() {                
        for (x, v) in chunk.iter().enumerate() {
            let chr = char::from_digit(*v, 36).unwrap();
            let coord = Coord::new(x, y);
            if coord == *current {
                print!("{}", chr.red());
            }
            else if path.iter().any(|c| **c == coord ) {
                print!("{}", chr.green());
            } else if open_set.iter().any(|osp| *osp == current) {
                print!("{}", chr.bright_white());
            } else {
                print!("{}", chr);
            }
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_char_to_num() {
        let chars: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
        let nums: Vec<u32> = chars.into_iter().map(|c| c.to_digit(36).unwrap() - 10).collect();
        let rhs: Vec<u32> = (0..=25).chain(0..=25).collect();
        assert_eq!(nums, rhs);
    }
}