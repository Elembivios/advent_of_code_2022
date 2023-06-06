use crate::utils::point::{Coord, Direction, Point};
use either::Either;

type C = Coord<usize>;

#[derive(Debug)]
enum Space {
    Empty,
    Pillar
}

impl Space {
    fn is_pillar(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Pillar => true
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Go(usize),
    Turn(Turn)
}

pub struct MonkeyMap {
    map: Vec<Point<usize, Space>>,
    instructions: Vec<Instruction>
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    L, R
}

impl Point<usize, Direction> {
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
        let (map, instructions) = data.split_once("\r\n\r\n").or_else(|| data.split_once("\n\n")).unwrap();

        let mut max_x: usize = 0;
        let max_y: usize = map.lines().count();

        let flattened_map: Vec<Point<usize, Space>> = map.lines().enumerate().map(|(y, l)| {
            if l.len() > max_x {
                max_x = l.len();
            }
            let row: Vec<(usize, char)> = l.chars().enumerate().skip_while(|(_x, c)| *c == ' ').collect();
            (y, row)
        }).flat_map(|(y, row)| {
            
            row.iter().map(|&(x, c)| {
                match c {
                    '#' => Point::new(x, y, Space::Pillar),
                    '.' => Point::new(x, y, Space::Empty),
                    _ => panic!("Invalid char for space {}", c)
                }
            }).collect::<Vec<Point<usize, Space>>>()            
        }).collect();

        let _side_size: usize = std::cmp::max(max_y, max_x) / 4;

        let instructions_str = instructions.lines().next().unwrap();
        let mut instructions: Vec<Instruction> = vec![];
        instructions_str.split_inclusive(&['R', 'L'][..]).for_each(|s| {
            
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

        Self {
            map: flattened_map,
            instructions
        }
    }

    fn part_01(&self) -> String {
        let mut current_position: Point<usize, Direction> = Point {
            coord: self.map[0].coord.clone(),
            value: Direction::E
        };
        let mut instructions_queue: Vec<Instruction> = self.instructions.iter().rev().cloned().collect();

        while let Some(instruction) = instructions_queue.pop() {
            match instruction {
                Instruction::Go(num_steps) => {                    
                    let (_steps_taken, next_position, _hit_pillar) = self.get_next_position(&current_position, num_steps);
                    current_position.coord = next_position;
                },
                Instruction::Turn(turn) => current_position.turn(turn)
            }
        }

        let facing: usize = match current_position.value {
            Direction::E => 0,
            Direction::S => 1,
            Direction::W => 2,
            Direction::N => 3,
            _ => unimplemented!()
        };

        let result = {
            1000 * (current_position.coord.y + 1) +
            4 * (current_position.coord.x + 1) +
            facing
        };
        result.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}

impl MonkeyMap {
    fn get_next_position(&self, current_position: &Point<usize, Direction>, num_steps: usize) -> (usize, C, bool) {
        let axis = &current_position.value.affected_axes()[0];
        let other_axis = axis.other();
        let map_it = match current_position.value {
            Direction::S | Direction::E => Either::Left(self.map.iter()),
            Direction::N | Direction::W => Either::Right(self.map.iter().rev()),
            _ => unimplemented!()
        };
        let mut next_position = None;
        either::for_both!(map_it, it => {
            let mut it = it
                .filter(|s| {
                    s.coord.get(&other_axis) == current_position.coord.get(&other_axis)
                })
                .cycle()
                .skip_while(|p| {
                    p.coord != current_position.coord
                })
                .enumerate()                
                .peekable();            

            while let Some((i, s)) = it.next() {
                if it.peek().and_then(|(_, next_s)| Some(next_s.value.is_pillar())).unwrap_or(false) {
                    next_position = Some((i, s.coord, true));
                    break;
                }
                if i < num_steps {
                    continue;
                }                    
                next_position = Some((i, s.coord, false));
                break;
            };            
        });
        next_position.unwrap()            
    }
}