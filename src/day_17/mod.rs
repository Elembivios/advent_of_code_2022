use std::fmt::Display;
use std::ops::Range;
use std::fs;
use anyhow::Context;

pub struct PyroclasticFlow {
    rock_shapes: Vec<Rock>,
    jet_patterns: Vec<char>,
}

impl crate::Advent for PyroclasticFlow {
    fn new(data: &str) -> Self {
        let filename = "src/day_17/rock_shapes.txt";
        let mut rock_shapes_str: &str = &fs::read_to_string(filename)
        .with_context(|| format!("Could not read {} file.", filename)).unwrap();
        rock_shapes_str = rock_shapes_str.trim_end();
        let mut rock_shapes: Vec<Rock> = rock_shapes_str.split("\r\n\r\n").map(|s| {            
            let mut width = 0;
            let chars = s.lines().map(|l| {
                let mut row: u8 = 0b0000_0000;
                for (i, c) in l.chars().rev().enumerate() {
                    if c == '#' {
                        row |= 2u8.pow(i as u32);
                    }                    
                }
                let curr_width = 8 - row.leading_zeros();
                if curr_width > width {
                    width = curr_width;
                }
                row
            }).collect();
            Rock::new(chars, width as usize)
        }).collect();       

        // Shifting all rocks 2 positions from left side
        for rock_shape in rock_shapes.iter_mut() {
            for row in rock_shape.structure.iter_mut() {
                *row <<= 8 - rock_shape.width - 3;
            }
        }
        let line = data.lines().next().unwrap();
        let jet_patterns = line.chars().collect();

        Self { rock_shapes, jet_patterns }
    }

    fn part_01(&self) -> String {
        let mut tower_builder = TowerBuilder::new();
        let height = tower_builder.add_rocks(2022, &self.jet_patterns, &self.rock_shapes);    
        height.to_string()        
    }

    fn part_02(&self) -> String {    
        let mut tower_builder = TowerBuilder::new();
        let height = tower_builder.add_rocks(1000000000000, &self.jet_patterns, &self.rock_shapes);
        height.to_string()
    }
}

type Map = Vec<u8>;

#[derive(Clone)]
struct Rock {
    structure: Map, 
    height: usize,
    width: usize
}

impl Rock {
    fn new(structure: Map, width: usize) -> Self {
        let height = structure.len();

        Self {
            structure,
            height,
            width
        }
    }

    fn tower_relative_iter(&self, rock_y: usize) -> impl Iterator<Item=(usize, &u8)> {
        self.structure.iter().rev().enumerate().map(move |(ri, row)| {
            let tower_y = rock_y + ri;
            (tower_y, row)
        })
    }
}


#[derive(Clone, Hash, Eq, PartialEq)]
struct Tower {
    data: Vec<u8>,
}

impl Tower {
    fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn spawn_height(&self) -> usize {
        self.height() + 3
    }

    fn is_rock_position_valid(&self, rock: &Rock, rock_y: isize) -> bool {
        rock.tower_relative_iter(rock_y as usize).all(|(ty, row)| {
            ty + 1 > self.height() || self.data[ty] & row == 0
        })
    }

    fn add_rock(&mut self, rock: &Rock, rock_y: usize) {
        for (ty, row) in rock.tower_relative_iter(rock_y as usize) {
            if ty + 1 > self.height() {
                self.data.push(*row)
            } else {
                self.data[ty] |= row;
            }            
        }
    }    

    fn check_for_pattern(&self) -> Option<Pattern> {
        // Returns (pattern_start_index, pattern_height) if the pattern is found 
        let mut pattern_range = None;
        let size = 100;
        let subset_it = self.data.windows(size).enumerate();
        'search: for (offset, subset) in subset_it {
            let mut superset_it = self.data.windows(size).enumerate().skip(offset + 1);
            let mut prev_i: Option<usize> = None;
            while let Some((i, superset)) = superset_it.next() {
                if superset == subset {
                    if let Some(prev_i_val) = prev_i {
                        pattern_range = Some( Pattern {
                            start_index: prev_i_val,
                            height: i - prev_i_val
                        });
                        break 'search;
                    } else {
                        prev_i = Some(i);
                        continue;
                    }
                }
            }
        }
        pattern_range
    }


    #[allow(dead_code)]
    fn display_with_rock(&self, rock: &Rock, rock_y: usize) {
        let mut clon = self.clone();
        if rock_y + rock.height > clon.height() {
            for _ in 0..(rock_y + rock.height) - clon.height() {
                clon.data.push(0)
            }
        }
        clon.add_rock(rock, rock_y);
        println!("{}", clon);
    }

    #[allow(dead_code)]
    fn display_range(&self, range: Range<usize>) {
        println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for row in self.data[range].iter().rev() {
            let s = u8_to_str(row);
            println!("{}", s);
        }
        println!("{:░<7}", "")
    }
}

impl Display for Tower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{esc}[2J{esc}[1;1H", esc = 27 as char)?;
        for row in self.data.iter().rev().take(200) {
            let s = u8_to_str(row);
            writeln!(f, "{}", s)?;
        }
        writeln!(f, "{:░<7}", "")
    }
}

fn u8_to_str(b: &u8) -> String {
    let mut b = *b;
    let mut s = String::new();
    for _n in (0..7).rev() {
        if b & 0b0100_0000 == 0b0100_0000 {
            s.push('█');
        } else {
            s.push('•');
        }
        b <<= 1;
    }
    s
}

struct TowerBuilder {
    tower: Tower,
    rocks_count: u64,
    jet_index: usize,
    virtual_height: u64,}

#[derive(Debug)]
struct Pattern {
    start_index: usize,
    height: usize
}

impl<'a> TowerBuilder {
    fn new() -> Self {
        Self {
            tower: Tower::new(),
            rocks_count: 0,
            jet_index: 0,
            virtual_height: 0,
        }
    }

    fn height(&self) -> u64 {
        self.virtual_height + self.tower.height() as u64
    }

    fn get_next_rock(&self, rock_shapes: &Vec<Rock>) -> Rock {
        rock_shapes[(self.rocks_count % rock_shapes.len() as u64) as usize].clone()
    }

    fn add_rocks(&mut self, rocks_limit: u64, jet_patterns: &Vec<char>, rock_shapes: &Vec<Rock>) -> u64 {
        let mut jet_patterns_it = jet_patterns.iter().skip(self.jet_index).enumerate().cycle();    

        let mut pattern_o: Option<Pattern> = None;
        let mut count_pattern_rocks_until_height: Option<u64> = None;
        let mut pattern_rock_count = 0;
        let mut done_pattern_check = false;

        'add_rock: while self.rocks_count < rocks_limit {       
            let mut rock = self.get_next_rock(rock_shapes).clone();
            // The rock is originally positioned at 2 spaces from left. No need to track that.
            let mut rock_y: isize = self.tower.spawn_height() as isize; 

            let last_jet_index = 'move_rock: loop {            
                let (jet_index, jet) = jet_patterns_it.next().unwrap();                
                // Check boundaries
                let in_boundries = match jet {
                    '>' => {!rock.structure.iter().any(|r| r.trailing_zeros() == 0)},
                    '<' => {!rock.structure.iter().any(|r| r.leading_zeros() < 2)},
                    _ => unreachable!()
                };
                if in_boundries {
                    // Create a clone and move it
                    let mut moved_rock = rock.clone();
                    match jet {
                        '>' => moved_rock.structure.iter_mut().for_each(|r| *r >>= 1),
                        '<' => moved_rock.structure.iter_mut().for_each(|r| *r <<= 1),
                        _ => unreachable!()
                    };         

                    // Check if newly moved value has a valid position in tower
                    if self.tower.is_rock_position_valid(&moved_rock, rock_y) {
                        // Replace with moved rock
                        rock = moved_rock;
                    }
                }
                
                if rock_y == 0 {
                    // Collides
                    break 'move_rock jet_index;                                        
                }

                let new_rock_y = rock_y - 1;
                if self.tower.is_rock_position_valid(&rock, new_rock_y) {
                    rock_y = new_rock_y;
                } else {
                    // Collides             
                    break 'move_rock jet_index;  
                }            
            };

            self.jet_index = last_jet_index;
            // Update tower
            self.tower.add_rock(&rock, rock_y as usize);            
            self.rocks_count += 1;                       

            if !done_pattern_check {
                if pattern_o.is_none() && self.rocks_count % 1000 == 0 {
                    pattern_o = self.tower.check_for_pattern();                                
                }
    
                if let Some(pattern) = pattern_o.as_ref() {
                    match count_pattern_rocks_until_height {
                        None => {
                            // Pass another pattern cycle, this time counting the number of rocks in it.
                            // .. Start to count rocks
                            let remainder = (self.tower.height() - pattern.start_index) % pattern.height;
                            if remainder < 4 {
                                count_pattern_rocks_until_height = Some(self.tower.height() as u64 + pattern.height as u64);
                            }
                        },
                        Some(height_limit) => {
                            let last_pattern_range = (height_limit - pattern.height as u64) as usize .. height_limit as usize;
                            let first_pattern_range = pattern.start_index .. pattern.start_index + pattern.height;
                            
                            if last_pattern_range.contains(&(rock_y as usize)) {
                                pattern_rock_count += 1;
                            }
                            if self.tower.data.len() < last_pattern_range.end {
                                continue 'add_rock;
                            }
    
                            // Check if pattern is complete / the same
                            if self.tower.data[last_pattern_range] == self.tower.data[first_pattern_range] {
                                let mul_times = (rocks_limit - self.rocks_count) / pattern_rock_count;
                                self.virtual_height += pattern.height as u64 * mul_times;
                                self.rocks_count += pattern_rock_count * mul_times;
                                done_pattern_check = true;
                                continue 'add_rock;
                            }
                        }
                    }             
                }
            }            
        }
        self.height()
    }
}