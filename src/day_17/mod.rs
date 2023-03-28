use std::{fs, ops::AddAssign, collections::VecDeque, thread::spawn};
use anyhow::Context;

const WIDTH: usize = 7;

pub struct PyroclasticFlow {
    rock_shapes: Vec<Grid>,
    jet_patterns: Vec<char>,
}

impl crate::Advent for PyroclasticFlow {
    fn new(data: &str) -> Self {
        let filename = "src/day_17/rock_shapes.txt";
        let mut rock_shapes_str: &str = &fs::read_to_string(filename)
        .with_context(|| format!("Could not read {} file.", filename)).unwrap();
        rock_shapes_str = rock_shapes_str.trim_end();
        let rock_shapes: Vec<Grid> = rock_shapes_str.split("\r\n\r\n").map(|s| {
            let chars = s.lines().map(|l| {
                l.chars().collect()
            }).collect();
            Grid::new(chars)
        }).collect();                

        let line = data.lines().next().unwrap();
        let jet_patterns = line.chars().collect();

        Self { rock_shapes, jet_patterns }
    }

    fn part_01(&self) -> String {
        let result = self.tower_height(2022);
        result.to_string()
    }

    fn part_02(&self) -> String {        
        2.to_string()
        // let result = self.tower_height(1000000000000);
        // result.to_string()
    }
}

impl PyroclasticFlow {
    fn tower_height(&self, limit: usize) -> usize {
        
        let mut jet_patterns_it = self.jet_patterns.iter().cycle();
        let mut tower: VecDeque<[char; WIDTH]> = VecDeque::new();
        tower.push_back(['.'; WIDTH]);
        tower.push_back(['.'; WIDTH]);
        tower.push_back(['.'; WIDTH]);

        let mut cutted_height = 0;

        let mut i = 0;
        let mut spawn_height = 3;
        while i < limit {
            if i % 100_000_000== 0 {
                println!("i: {}", i);
            }
            let rock = &self.rock_shapes[i % self.rock_shapes.len()];
            let mut rock_pos = (2, spawn_height as i32);

            // Add new empty rows to tower
            let num_row_to_add = spawn_height + 4 - tower.len();
            for _ in 0..num_row_to_add {
                tower.push_back(['.'; WIDTH]);
            }

            // print_tower(&tower);
            let mut stopped = false;

            while !stopped {                
                let jet = jet_patterns_it.next().unwrap();
                // println!("Rock pos: {:?}, jet: {}", rock_pos, jet);
                let new_rock_pos = match jet {
                    '>' => {
                        (rock_pos.0 + 1, rock_pos.1)
                    },
                    '<' => {
                        (rock_pos.0 - 1, rock_pos.1)
                    },
                    _ => unreachable!()
                };

                if rock.can_move_to(&tower, &new_rock_pos) {
                    rock_pos = new_rock_pos;
                }
                let new_rock_pos = (rock_pos.0, rock_pos.1 - 1);
                if rock.can_move_to(&tower, &new_rock_pos) {
                    rock_pos = new_rock_pos;
                } else {
                    stopped = true;

                    // Update tower
                    for (cx, cy, value) in rock.coord_it(&(rock_pos.0 as usize, rock_pos.1 as usize)) {
                        if *value == '#' {
                            *tower.get_mut(cy).unwrap().get_mut(cx).unwrap() = *value;
                        }                        
                    }

                    for y in (rock_pos.1 as usize..rock_pos.1 as usize + rock.height).rev() {                        
                        if tower[y].iter().all(|c| *c == '#') {
                            // print_tower_with_rock(&tower, rock, &(rock_pos.0 as usize, rock_pos.1 as usize));
                            // println!("Cutting tower at: {}", y);
                            for _ in 0..y {
                                tower.pop_front();
                            }
                            rock_pos.1 -= y as i32;
                            spawn_height -= y;
                            cutted_height += y;
                        }
                        break;
                    }

                    let new_height = rock_pos.1 as usize + 3 + rock.height;
                    if new_height > spawn_height {
                        spawn_height = new_height;
                    }
                }            
                // println!("New rock pos: {:?}", rock_pos);
            }

            i += 1;
        }

        cutted_height + spawn_height - 3
    }
}

type Map = Vec<Vec<char>>;

struct Grid {
    structure: Map, 
    width: usize,
    height: usize
}

impl Grid {
    fn new(structure: Map) -> Self {
        let width = structure.iter().map(|r| r.len()).max().unwrap();
        let height = structure.len();

        Self {
            structure,
            width,
            height
        }
    }

    fn coord_it<'a>(&'a self, position: &'a (usize, usize)) -> impl IntoIterator<Item = (usize, usize, &char)> + 'a{
        self.structure.iter().rev().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().map(move |(x, value)| {
                (x + position.0,  y + position.1, value)
            })
        })
    }

    fn can_move_to(&self, tower: &VecDeque<[char; WIDTH]>, (x, y): &(i32, i32)) -> bool {
        if *x < 0 || *y < 0  {
            return false;
        }
        let x: usize = *x as usize;
        let y: usize = *y as usize;

        if x + self.width > WIDTH {
            return false;
        }

        for (cx, cy, value) in self.coord_it(&(x, y)) {
            if *value == '.' {
                continue;
            }
            if tower[cy][cx] == '#' {                    
                return false;
            }
        }
        true
    }
    
}



fn print_tower(tower: &VecDeque<[char; WIDTH]>) {
    for row in tower.iter().rev() {
        let s: String = row.into_iter().collect();
        println!("{}", s);
    }
    println!("----------");
}


fn print_tower_with_rock(tower: &VecDeque<[char; WIDTH]>, rock: &Grid, rock_pos: &(usize, usize)) {
    // Clear terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut tower = tower.clone();
    for (cx, cy, value) in rock.coord_it(rock_pos) {
        if *value == '#' {
            *tower.get_mut(cy).unwrap().get_mut(cx).unwrap() = '@';
        }
    }
    print_tower(&tower);
    println!("----------");
}