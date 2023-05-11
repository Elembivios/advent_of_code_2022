use std::ops::Range;

use crate::utils::point::{Coord, Direction, Point};


pub struct MonkeyMap {
    pillars: Vec<Coord<usize>>,
    x_map: Vec<Range<usize>>,
    y_map: Vec<Range<usize>>,
    instructions: Vec<Instruction>
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    L, R
}

#[derive(Debug)]
enum Instruction {
    Go(usize),
    Turn(Turn)
}

impl Point<usize, Direction> {
    fn go(&mut self, num: usize) {
        use Direction::*;
        match self.value {
            N => self.coord.y - num,
            E => self.coord.x + num,
            S => self.coord.y + num,
            W => self.coord.x - num,
            _ => unimplemented!()
        };
    }
    fn turn(&mut self, turn: Turn) {
        use Turn::*;
        use Direction::*;

        self.value = match turn {
            L => {
                match self.value {
                    N => W,
                    E => N,                    
                    S => E,
                    W => S,
                    _ => unimplemented!()
                }
            },
            R => {
                match self.value {
                    N => E,
                    E => S,
                    S => W,
                    W => N,
                    _ => unimplemented!()
                }
            }
        };        
    }
}

impl crate::Advent for MonkeyMap {
    fn new(data: &str) -> Self
        where 
            Self: Sized {

        let mut pillars: Vec<Coord<usize>> = vec![];
        let mut x_map = vec![];      
        println!("DATA: {:?}", data);
        let (map, instructions) = data.split_once("\r\n\r\n").or_else(|| data.split_once("\n\n")).unwrap();

        map.lines().enumerate().map(|(x, l)| {
            let row: Vec<(usize, char)> = l.chars().enumerate().skip_while(|(_y, c)| *c == ' ').collect();
            (x, row)
        }).for_each(|(x, row)| {
            println!("Row: {:?}", row);
            row.iter().for_each(|&(y, c)| {
                if c == '#' {
                    let pillar: Coord<usize> = Coord::new(x, y);
                    pillars.push(pillar);
                }
            });
            let start = row[0].0;
            let end = row[row.len() - 1].0;
            x_map.push(start..end);
        });

        let width = x_map.first().unwrap().len();

        let mut y_map_start: Vec<usize> = (0..width).map(|_| usize::MAX).collect();
        let mut y_map_end: Vec<usize> = (0..width).map(|_| usize::MIN).collect();

        for (x, rng) in x_map.iter().enumerate() {
            y_map_start.iter_mut().enumerate().filter(|(y, n)| rng.contains(&y) && x < **n).for_each(|(_y, n)| *n = x);
            y_map_end.iter_mut().enumerate().filter(|(y, n)| rng.contains(&y) && x > **n).for_each(|(_y, n)| *n = x);
        }

        let y_map = y_map_start.iter().zip(y_map_end).map(|(start, end)| (*start..end)).collect();

        let instructions_str = instructions.lines().next().unwrap();
        let mut instructions: Vec<Instruction> = vec![];
        instructions_str.split_inclusive(&['R', 'L'][..]).for_each(|s| {
            println!("S: {}", s);
            
            let turn = match &s[s.len() - 1..s.len()] {
                "L" => Some(Turn::L),
                "R" => Some(Turn::R),
                _ => None
            };            
            let num_end = match turn {
                Some(_) => s.len() - 1,
                None => s.len()
            };
            
            let num = Instruction::Go(s[0..num_end].parse().unwrap());
            instructions.push(num);
            if let Some(turn) = turn {
                instructions.push(Instruction::Turn(turn));
            }            
        });
        println!("Instructions: {:?}", instructions);


        Self {
            pillars: pillars,
            x_map: x_map,
            y_map: y_map,
            instructions
        }
    }

    fn part_01(&self) -> String {
        let mut current_position: Point<usize, Direction> = Point::new(0, 0, Direction::E);
        for instruction in &self.instructions {
            match instruction {
                Instruction::Go(num) => {
                    // let pillars_in_the_way = self.pillars.iter().filter(|p| {
                    //     let axis = current_position.value.affected_axes()[0];

                    // })
                    current_position.go(*num)
                },
                Instruction::Turn(turn) => current_position.turn(*turn)
            }
        }
        1.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}