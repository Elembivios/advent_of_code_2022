use crate::utils::point::{Coord, Point, Grid};
use std::collections::HashMap;
use std::io;
use owo_colors::{OwoColorize};

type P = Point<usize, u32>;
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
        let map: Vec<Vec<P>> = data
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let val = match c {
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
                        };
                        Point::new(x, y, val)
                    })
                .collect()
            }).collect();

        let grid: Grid<u32> = Grid::new(map);


        HillClimbingAlhorithm {
            grid,
            start: start.into(),
            end: end.into(),
        }
    }

    fn part_01(&self) -> String {
        let heuristic = |current: &P, goals: &Vec<&P>| {
            goals.iter().map(|goal| {
                let x = goal.coord.x.abs_diff(current.coord.x);
                let y = goal.coord.y.abs_diff(current.coord.y);
                let future_path = x + y;
                future_path
            }).min().unwrap()            
        };
        let valid_neighbour = |current: &P, neighbour: &P| {
            (neighbour.value as i32 - current.value as i32) <= 1
        };

        let path = a_star(
            &self.grid, 
            self.grid.get_point(&self.start), 
            vec![self.grid.get_point(&self.end)], 
            heuristic, 
            valid_neighbour
        );
                
        let path = path.unwrap_or(vec![]);
        let cost = path.iter().count().checked_sub(1).unwrap_or(0);
        cost.to_string()
    }

    fn part_02(&self) -> String {
        let heuristic = |current: &P, goals: &Vec<&P>| {
            goals.iter().map(|goal| {
                let x = goal.coord.x.abs_diff(current.coord.x);
                let y = goal.coord.y.abs_diff(current.coord.y);
                let future_path = x + y;
                future_path
            }).min().unwrap()            
        };
        let valid_neighbour = |current: &P, neighbour: &P| {
            (neighbour.value as i32 - current.value as i32) >= -1
        };

        let goals: Vec<_> = self.grid.map.iter().filter(|p| p.value == 0).collect();
        let path = a_star(
            &self.grid, 
            self.grid.get_point(&self.end), 
            goals, 
            heuristic,
            valid_neighbour
        );
        let path = path.unwrap_or(vec![]);
        let cost = path.iter().count().checked_sub(1).unwrap_or(0);
        cost.to_string()
    }
}


fn reconstruct_path<'a>(came_from: &HashMap<&'a P, &'a P>, current: &'a P) -> Vec<&'a P> {
    let mut total_path: Vec<&P> = vec![current];
    let mut previous_point = came_from.get(current);
    while previous_point.is_some() {
        if let Some(p) = previous_point {
            total_path.insert(0, p.clone());
            previous_point = came_from.get(p);
        }
    }
    total_path
}

fn d(_current: &P, _neighbour: &P) -> usize {
    1
}

fn a_star<'a>(grid: &'a Grid<u32>, start: &'a P, goals: Vec<&'a P>, heuristic: fn(&P, &Vec<&P>) -> usize, valid_neighbour: fn(&P, &P) -> bool) -> Option<Vec<&'a P>> {
    let mut open_set = vec![start];
    let mut came_from: HashMap<&'a P, &'a P> = HashMap::new();
    let mut g_score: HashMap<&C, usize> = grid.map.iter().map(|p| (&p.coord, usize::MAX)).collect();
    g_score.insert(&start.coord, 0);

    let mut f_score: HashMap<&C, usize> = grid.map.iter().map(|p| (&p.coord, usize::MAX)).collect();
    f_score.insert(&start.coord, heuristic(&start, &goals));

    while open_set.len() != 0 {
        let min_f = open_set.iter().map(|p| {
            (p, f_score.get(&p.coord).unwrap())
        }).min_by(|a, b| {
            a.1.cmp(b.1)
        }).unwrap();
        let current = *min_f.0;

        if goals.iter().any(|g| g.coord == current.coord) {
            return Some(reconstruct_path(&came_from, current));            
        }

        let pos = open_set.iter().position(|p| *p == current).unwrap();
        open_set.remove(pos);

        for neighbour in grid.get_neighbours(&current.coord) {
            if !valid_neighbour(current, neighbour) {
                continue;
            }
            let tentative_g_score = g_score[&current.coord] + d(current, neighbour);

            if tentative_g_score < g_score[&neighbour.coord] {
                came_from.insert(neighbour, current);
                let tentative_f_score = tentative_g_score + heuristic(neighbour, &goals);
                g_score.insert(&neighbour.coord, tentative_g_score);
                f_score.insert(&neighbour.coord, tentative_f_score);
                if !open_set.contains(&neighbour) {
                    open_set.push(neighbour);
                }
            }
        }
        // display_and_wait(grid, &open_set, current, &reconstruct_path(&came_from, current));
    }
    None
}

#[allow(dead_code)]
fn display_and_wait(grid: &Grid<u32>, open_set: &Vec<&P>, current: &P, path: &Vec<&P>) {    
    display(grid, open_set, current, path);
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).ok().unwrap(); 
    print!("{esc}c", esc = 27 as char);
}

#[allow(dead_code)]
fn display(grid: &Grid<u32>, open_set: &Vec<&P>, current: &P, path: &Vec<&P>) {    
    for chunk in grid.map.chunks(grid.width) {        
        for p in chunk {
            let chr = char::from_digit(p.value, 36).unwrap();
            if p.coord == current.coord {
                print!("{}", chr.red());
            }
            else if path.iter().map(|pp| &pp.coord).any(|c| *c == p.coord ) {
                print!("{}", chr.green());
            } else if open_set.iter().map(|osp| &osp.coord).any(|osp| *osp == p.coord) {
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