use itertools::Itertools;

use crate::utils::point::{Coord, Grid};
use std::{cmp, fmt, iter};

// 24376 -- too low


#[derive(Debug, PartialEq, Clone)]
enum Material {
    Rock,
    Sand,
    Air
}

impl Material {
    fn is_empty(&self) -> bool {
        match self {
            Material::Air => true,
            _ => false
        }
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Material::Air => write!(f, "."),
            Material::Sand => write!(f, "O"),
            Material::Rock => write!(f, "#")
        }
    }
}

pub struct RegolithReservoir {
    grid: Grid<Material>,
    offset_x: usize,
}

impl crate::Advent for RegolithReservoir {
    fn new(data: &str) -> Self {
        let mut rocks: Vec<Vec<Coord<usize>>> = data
            .lines()
            .map(|l| {
                l.split(" -> ").map(|line| {
                    let mut it = line.split(",").map(|v| v.parse().unwrap());
                    let x = it.next().unwrap();
                    let y = it.next().unwrap();
                    Coord::new(x, y)
                }).collect()
            }).collect();
        let start = Coord::new(500, 0);
        let it = iter::once(&start).chain(rocks.iter().flatten());
        let mut y_sorted = it.clone().sorted_by(|a, b| a.y.cmp(&b.y));
        let mut x_sorted = it.sorted_by(|a, b| a.x.cmp(&b.x));
        let offset_y = y_sorted.next().unwrap().y;
        let max_y = y_sorted.last().unwrap().y;        
        let offset_x = x_sorted.next().unwrap().x - 1;
        let max_x = x_sorted.last().unwrap().x + 1;        
        let mut map: Vec<Vec<Material>> = (0..=max_y-offset_y).map(|_| (0..=max_x-offset_x).map(|_|Material::Air).collect()).collect();

        rocks.iter_mut().flatten().for_each(|c| {
            c.x -= offset_x;
            c.y -= offset_y;
        });

        for rock_formation in rocks {
            let mut it = rock_formation.iter().peekable();
            'rock_path: while let Some(rock_start) = it.next() {
                let rock_end = it.peek();
                let Some(rock_end) = rock_end else {
                    break 'rock_path;
                };
                if rock_start.x == rock_end.x {
                    // Vertical
                    let c_min_y = cmp::min(rock_start.y, rock_end.y);
                    let c_max_y = cmp::max(rock_start.y, rock_end.y);
                    for y in c_min_y..=c_max_y {
                        let v = &mut map[y][rock_start.x];
                        *v = Material::Rock;
                    }
                } else {
                    // Horizontal
                    let c_min_x = cmp::min(rock_start.x, rock_end.x);
                    let c_max_x = cmp::max(rock_start.x, rock_end.x);
                    for x in c_min_x..=c_max_x {
                        let v = &mut map[rock_start.y][x];
                        *v = Material::Rock;
                    }
                }
            }
        }
        let grid = Grid::new(map);
        RegolithReservoir { 
            grid,
            offset_x,
        }
    }

    fn part_01(&self) -> String {        
        let start = Coord::new(500 - self.offset_x, 0);
        let mut grid = self.grid.clone();

        let mut sand_counter = 0;
        while let Some(rest_pos) = search_next_rest_pos(&mut grid, &start) {
            sand_counter += 1;
            *grid.get_val_mut(&rest_pos) = Material::Sand;
        }
        sand_counter.to_string()
    }

    fn part_02(&self) -> String {
        // Plus four becouse of 2 extra rows we'll be adding, and one one each side for overflow        
        let desired_width = (self.grid.height + 4) * 3; 
        let to_add = (desired_width - self.grid.width) / 2;
        let width = to_add * 2 + self.grid.width;
        println!("Desired width: {}, Width: {}", desired_width, width);
        
        let two_rows: Vec<Vec<Material>> = (0..2).map(|y| {
            (0..width).map(|_| {
                if y == 1{
                    Material::Rock
                } else {
                    Material::Air
                }
            }).collect()
        }).collect();

        let map: Vec<Vec<Material>> = self.grid.map
            .chunks(self.grid.width)
            .map(|chunk| {
                let lhs_add: Vec<Material> = (0..to_add).map(|_| Material::Air).collect();
                let rhs_add: Vec<Material> = (to_add + self.grid.width..2 * to_add + self.grid.width).map(|_| Material::Air).collect();
                lhs_add.into_iter().chain(chunk.iter().cloned()).chain(rhs_add.into_iter()).collect()            
            })
            .chain(two_rows.iter().cloned())
            .collect();

        let start = Coord::new(500 - self.offset_x + to_add, 0);
        
        let mut grid = Grid::new(map);
        
        let mut sand_counter = 0;
        while let Some(rest_pos) = search_next_rest_pos(&mut grid, &start) {
            sand_counter += 1;
            *grid.get_val_mut(&rest_pos) = Material::Sand;
        }
        (sand_counter + 1).to_string()
    }
}

fn search_next_rest_pos(grid: &mut Grid<Material>, start: &Coord<usize>) -> Option<Coord<usize>> {
    let mut current = start.clone();     
    let mut rest_pos: Option<Coord<usize>> = None;
    'search_rest: while rest_pos.is_none() {                                
        let val = grid.get_val(&current);
        if val.is_empty() {
            if current.y == grid.height - 1 {
                return rest_pos;
            }
            current.y += 1;                    
            continue 'search_rest;
        }
        let left_val = grid.get_val(&Coord::new(current.x - 1, current.y));
        if left_val.is_empty() {                          
            current.x -= 1;
            continue 'search_rest;
        }
        let right_val = grid.get_val(&Coord::new(current.x + 1, current.y));
        if right_val.is_empty() {
            current.x += 1;
            continue 'search_rest;                                        
        }    
        current.y -= 1;

        if current == *start {
            return None;
        }
        rest_pos = Some(current.clone());                
    }
    rest_pos    
    
}