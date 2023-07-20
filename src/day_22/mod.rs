use itertools::Itertools;
use anyhow::{Result, Error, anyhow};
use crate::utils::point::{Coord, Direction, Point, Grid};
use std::fmt;
use std::collections::VecDeque;

type C = Coord<usize>;

#[derive(Debug, PartialEq, Clone)]
enum Space {
    Void,
    Empty,
    Pillar
}

impl Space {
    fn is_pillar(&self) -> bool {
        match self {            
            Self::Void | Self::Empty => false,
            Self::Pillar => true
        }
    }
}

impl TryFrom<char> for Space {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' => Ok(Self::Void),
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Pillar),
            _ => Err(anyhow!("Invalid char {} received when constructing space!", value))
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Space::Empty => write!(f, "."),
            Space::Void => write!(f, " "),
            Space::Pillar => write!(f, "#")
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Go(usize),
    Turn(Turn)
}

pub struct MonkeyMap {
    sides: Grid<Option<Grid<Space>>>,
    flattened_map: Grid<Space>,
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

        let height: usize = map.lines().count();
        let width: usize = map.lines().map(|r| r.len()).max().unwrap();
        let side_size = std::cmp::max(height, width) / 4;

        let sides: Vec<Vec<Grid<Space>>> = map.lines().chunks(side_size).into_iter().map(|slice| {            
            let mut slice_map: Vec<Vec<Vec<Space>>> = vec![
                vec![
                    Vec::with_capacity(side_size); side_size
                ]; width / side_size
            ];
            slice.into_iter().enumerate().for_each(|(y, l)| {
                l.chars().chunks(side_size).into_iter().enumerate().for_each(|(index_x, part)| {
                    let row_part: Vec<Space> = part.into_iter().map(|c| {
                        Space::try_from(c).unwrap()
                    }).collect();
                    slice_map[index_x][y] = row_part
                })
            });
            slice_map.into_iter().map(|side_map| {
                Grid::new(side_map)
            }).collect()
        }).collect();

        let sides: Grid<Option<Grid<Space>>> = Grid::new(sides.into_iter().map(|r| {
            r.into_iter().map(|g| {
                if *g.map.get(0).unwrap_or(&Space::Void) == Space::Void {
                    None
                } else {
                    Some(g)
                }
            }).collect()
        }).collect());

        let flattened_map: Vec<Vec<Space>> = map.lines().map(|l| {
            let mut chars_it = l.chars();
            (0..width).map(|_x| {
                let c = chars_it.next().unwrap_or(' ');
                Space::try_from(c).unwrap()
            }).collect()
        }).collect();

        // let flattened_map: Vec<Vec<Space>> = map.lines().map(|l| {
        //     l.chars().map(|c| {
        //         Space::try_from(c).unwrap()
        //     }).collect()
        // }).collect();

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
            sides,
            flattened_map: Grid::new(flattened_map),
            instructions
        }
    }

    fn part_01(&self) -> String {
        let current_coord: C = Coord::new(
            self.flattened_map.map.iter().enumerate().skip_while(|s| *s.1 == Space::Void).map(|s| s.0).next().unwrap(),
            0
        );
        let mut current_position: Point<usize, Direction> = Point {
            coord: current_coord,
            value: Direction::E
        };
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Go(num_steps) => {                    
                    let (_steps_taken, next_position, _hit_pillar) = self.get_next_position(&current_position, *num_steps);
                    current_position.coord = next_position;                    
                },
                Instruction::Turn(turn) => current_position.turn(*turn)
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
        let mut current_side_coord = self.sides.iter_coords().filter(|c: &C| self.sides.get_val(c).is_some()).next().unwrap();
        let mut current_inner_position: Point<usize, Direction> = Point {
            coord: Coord::new(0, 0),
            value: Direction::E
        };
        let mut instructions_queue = VecDeque::from(self.instructions.clone());
        while let Some(instruction) = instructions_queue.pop_front() {
            match instruction {
                Instruction::Go(num_steps) => {
                    let current_side = self.sides.get_val(&current_side_coord).as_ref().unwrap();                    
                    let (steps_taken, next_position, hit_pillar) = self.get_next_side_position(
                        current_side_coord, &current_inner_position, num_steps);                    
                    let steps_diff = num_steps - steps_taken;
                    if steps_diff == 0 || hit_pillar == true {
                        // Did all the steps on this plane / side or hit a pillar and 
                        // came to a stop.                                       
                        current_inner_position.coord = next_position;
                    } else {
                        // Switching planes / side
                        let (new_side_coord, side_rotation) = self.get_next_side(
                            &current_side_coord, current_inner_position.value);                        
                        let c = current_inner_position.coord;
                        let d = current_inner_position.value;

                        // Position coordinate to the opposide side and flip if necessary                      
                        let mut new_c: C = match d {
                            Direction::N => {Coord::new(c.x, current_side.height - 1)},
                            Direction::E => {Coord::new(0, c.y)},
                            Direction::S => {Coord::new(c.x, 0)},
                            Direction::W => {Coord::new(current_side.width - 1, c.y)},
                            _ => unreachable!()
                        };
                        // Rotate coordinate based on required new side rotation
                        match side_rotation {
                            0 => {},
                            -90 => (new_c.x, new_c.y) = (new_c.y, current_side.height - 1 - new_c.x),
                            90 => (new_c.x, new_c.y) = (current_side.width - 1 - new_c.y, new_c.x),
                            180 | -180 => (new_c.x, new_c.y) = (current_side.width - 1 - new_c.x, current_side.height - 1 - new_c.y),
                            _ => unreachable!()
                        }

                        // Change direction
                        let new_dir = d.rotate(side_rotation);

                        // Check if next coord on new side is a pillar!
                        if !self.sides.get_val(&new_side_coord).as_ref().unwrap().get_val(&new_c).is_pillar() {                            
                            current_inner_position.coord = new_c;
                            current_inner_position.value = new_dir;
                            current_side_coord = new_side_coord;
                            if steps_diff > 1 {
                                // One step is required to switch planes / sides
                                instructions_queue.push_front(Instruction::Go(steps_diff - 1));
                            }                            
                        } else {
                            // Next coordinate on the next side / plane is a pillar
                            current_inner_position.coord = next_position;
                        }
                    }
                },
                Instruction::Turn(turn) => {
                    current_inner_position.turn(turn)
                }
            }
        }
        let facing: usize = match current_inner_position.value {
            Direction::E => 0,
            Direction::S => 1,
            Direction::W => 2,
            Direction::N => 3,
            _ => unimplemented!()
        };

        let x_mul = self.sides.get_val(&current_side_coord).as_ref().unwrap().width * current_side_coord.x;
        let y_mul =  self.sides.get_val(&current_side_coord).as_ref().unwrap().height * current_side_coord.y;

        let x = current_inner_position.coord.x + 1 + x_mul;
        let y = current_inner_position.coord.y + 1 + y_mul;
        
        let result = {
            1000 * y +
            4 * x +
            facing
        };
        result.to_string()
    }
}

impl MonkeyMap {
    fn get_next_position(&self, current_position: &Point<usize, Direction>, num_steps: usize) -> (usize, C, bool) {
        let mut it = self.flattened_map
            .wrapped_direction_iter(current_position.value, current_position.coord)
            .filter(|c| {
                *self.flattened_map.get_val(c) != Space::Void
            })
            .enumerate();
        let mut previous_coord = current_position.coord;
        while let Some((i, c)) = it.next() {       
            if self.flattened_map.get_val(&c).is_pillar() {
                return (i, previous_coord, true);
            } else if i + 1 == num_steps {
                return (i, c, false);
            }
            previous_coord = c;            
        }
        unreachable!("No next coord found! This shouldn't happen.")     
    }

    fn get_next_side_position(&self, side_coord: C, current_position: &Point<usize, Direction>, num_steps: usize) -> (usize, C, bool) {
        let side = self.sides.get_val(&side_coord).as_ref().unwrap();
        let it = side
            .direction_iter(current_position.value, current_position.coord);
        let mut previous_coord = current_position.coord;
        let mut i = 0;
        for c in it {
            if side.get_val(&c).is_pillar() {
                return (i, previous_coord, true);
            } else if i + 1 == num_steps {
                return (i + 1, c, false);
            }
            previous_coord = c;
            i += 1;
        }
        return (i, previous_coord, false);   
    }

    fn exists(&self, c: Option<C>) -> Option<C> {
        match c {
            Some(c) => {
                if self.sides.get_val(&c).is_some() {
                    return Some(c)
                }       
            }, 
            None => {}
        }
        None
    }

    fn get_next_side(&self, c: &C, direction: Direction) -> (C, isize) {
        // Forward coord
        let fc = self.sides.get_neighbour(c, direction);
        match self.exists(fc) {
            Some(fc) => return (fc, 0),
            None => {}
        }

        let lc = self.sides.get_neighbour(c, direction.rotate(-90));
        match self.exists(lc) {
            Some(lc) => {
                let lfc = self.sides.get_neighbour(c, direction.rotate(-45));
                match self.exists(lfc) {
                    Some(lfc) => return (lfc, -90),
                    _ => {}
                }

                let llc = self.sides.get_neighbour(&lc, direction.rotate(-90));
                match self.exists(llc) {
                    Some(llc) => {
                        let llfc = self.sides.get_neighbour(&llc, direction);
                        match self.exists(llfc) {
                            Some(llfc) => return (llfc, 180),
                            None => {}
                        }
                    },
                    None => {}
                }

                let lbc = self.sides.get_neighbour(&lc, direction.rotate(180));
                match self.exists(lbc) {
                    Some(lbc) => {
                        let lblc = self.sides.get_neighbour(&lbc, direction.rotate(-90));
                        match self.exists(lblc) {
                            Some(lblc) => {
                                let lbllc = self.sides.get_neighbour(&lblc, direction.rotate(-90));
                                match self.exists(lbllc) {
                                    Some(lbllc) => return (lbllc, 90),
                                    None => {}
                                }
                            },
                            None => {}
                        }
                        let lbbc = self.sides.get_neighbour(&lbc, direction.rotate(180));
                        match self.exists(lbbc) {
                            Some(lbbc) => {
                                let lbblc = self.sides.get_neighbour(&lbbc, direction.rotate(-90));
                                match self.exists(lbblc) {
                                    Some(lbblc) => {
                                        let lbblbc = self.sides.get_neighbour(&lbblc, direction.rotate(180));
                                        match self.exists(lbblbc) {
                                            Some(lbblbc) => return (lbblbc, 0),
                                            None => {}
                                        }
                                    },
                                    None => {}
                                }
                            },
                            None => {}
                        }
                    },
                    None => {}
                }
            },
            None => {}
        }

        let rc = self.sides.get_neighbour(c, direction.rotate(90));
        match self.exists(rc) {
            Some(rc) => {
                let rfc = self.sides.get_neighbour(c, direction.rotate(45));
                match self.exists(rfc) {
                    Some(rfc) => return (rfc, 90),
                    _ => {}
                }

                let rrc = self.sides.get_neighbour(&rc, direction.rotate(90));
                match self.exists(rrc) {
                    Some(rrc) => {
                        let rrfc = self.sides.get_neighbour(&rrc, direction);
                        match self.exists(rrfc) {
                            Some(rrfc) => return (rrfc, -180),
                            None => {}
                        }
                    },
                    None => {}
                }

                let rbc = self.sides.get_neighbour(&rc, direction.rotate(180));
                match self.exists(rbc) {
                    Some(rbc) => {
                        let rbrc = self.sides.get_neighbour(&rbc, direction.rotate(90));
                        match self.exists(rbrc) {
                            Some(rbrc) => {
                                let rbrrc = self.sides.get_neighbour(&rbrc, direction.rotate(90));
                                match self.exists(rbrrc) {
                                    Some(rbrrc) => return (rbrrc, -90),
                                    None => {}
                                }
                            },
                            None => {}
                        }

                        let rbbc = self.sides.get_neighbour(&rbc, direction.rotate(180));
                        match self.exists(rbbc) {
                            Some(rbbc) => {
                                let rbbrc = self.sides.get_neighbour(&rbbc, direction.rotate(90));
                                match self.exists(rbbrc) {
                                    Some(rbbrc) => {
                                        let rbbrbc = self.sides.get_neighbour(&rbbrc, direction.rotate(180));
                                        match self.exists(rbbrbc) {
                                            Some(rbbrbc) => return (rbbrbc, 0),
                                            None => {}
                                        }
                                    },
                                    None => {}
                                }
                            },
                            None => {}
                        }
                    },
                    None => {}
                }
            },
            None => {}
        }


        let bc = self.sides.get_neighbour(c, direction.rotate(180));
        match self.exists(bc) {
            Some(bc) => {
                let blc = self.sides.get_neighbour(&bc, direction.rotate(-90));
                match self.exists(blc) {
                    Some(blc) => {
                        let bllc = self.sides.get_neighbour(&blc, direction.rotate(-90));
                        match self.exists(bllc) {
                            Some(bllc) => {
                                return (bllc, -180)
                            },
                            None => {}
                        }

                        let blbc = self.sides.get_neighbour(&blc, direction.rotate(180));
                        match self.exists(blbc) {
                            Some(blbc) => {
                                let blbbc = self.sides.get_neighbour(&blbc, direction.rotate(180));
                                match self.exists(blbbc) {
                                    Some(blbbc) => {
                                        let blbblc = self.sides.get_neighbour(&blbbc, direction.rotate(-90));
                                        match self.exists(blbblc) {
                                            Some(blbblc) => return (blbblc, 0),
                                            None => {}
                                        }
                                    },
                                    None => {}
                                }
                            },
                            None => {}
                        }

                    },
                    None => {}
                }
                let brc = self.sides.get_neighbour(&bc, direction.rotate(90));
                match self.exists(brc) {
                    Some(brc) => {
                        let brrc = self.sides.get_neighbour(&brc, direction.rotate(90));
                        match self.exists(brrc) {
                            Some(brrc) => {
                                return (brrc, 180)
                            },
                            None => {}
                        }
                        let brbc = self.sides.get_neighbour(&brc, direction.rotate(180));
                        match self.exists(brbc) {
                            Some(brbc) => {
                                let brbbc = self.sides.get_neighbour(&brbc, direction.rotate(180));
                                match self.exists(brbbc) {
                                    Some(brbbc) => {
                                        let brbbrc = self.sides.get_neighbour(&brbbc, direction.rotate(90));
                                        match self.exists(brbbrc) {
                                            Some(brbbrc) => return (brbbrc, 0),
                                            None => {}
                                        }
                                    },
                                    None => {}
                                }
                            },
                            None => {}
                        }
                    },
                    None => {}
                }

                let bbc = self.sides.get_neighbour(&bc, direction.rotate(180));
                match self.exists(bbc) {
                    Some(bbc) => {
                        let bblc = self.sides.get_neighbour(&bbc, direction.rotate(-90));
                        match self.exists(bblc) {
                            Some(bblc) => {
                                let bblbc = self.sides.get_neighbour(&bblc, direction.rotate(180));
                                match self.exists(bblbc) {
                                    Some(bblbc) => return (bblbc, 90),
                                    None => {}
                                }
                            },
                            None => {}
                        }
                        let bbrc = self.sides.get_neighbour(&bbc, direction.rotate(90));
                        match self.exists(bbrc) {
                            Some(bbrc) => {
                                let bbrbc = self.sides.get_neighbour(&bbrc, direction.rotate(180));
                                match self.exists(bbrbc) {
                                    Some(bbrbc) => return (bbrbc, -90),
                                    None => {}
                                }
                            },
                            None => {}
                        }
                    },
                    None => {}
                }
            },
            None => {}            
        }
        unreachable!("The side for {} in direction {:?} could not be found", c, direction);
    }
}


#[cfg(test)]
mod tests {
    #[test] 
    fn test_modulo() {
        assert_eq!(0i32.rem_euclid(4), 0);
        assert_eq!(1i32.rem_euclid(4), 1);
        assert_eq!(2i32.rem_euclid(4), 2);
        assert_eq!(3i32.rem_euclid(4), 3);
        assert_eq!(4i32.rem_euclid(4), 0);
        assert_eq!(5i32.rem_euclid(4), 1);
        assert_eq!((-1i32).rem_euclid(4), 3);
        assert_eq!((-2i32).rem_euclid(4), 2);
        assert_eq!((-3i32).rem_euclid(4), 1);
        assert_eq!((-4i32).rem_euclid(4), 0);
    }

    #[test]
    fn test_switch_dir() {
        assert_eq!((0usize + 2).rem_euclid(4), 2);
        assert_eq!((1usize + 2).rem_euclid(4), 3);
        assert_eq!((2usize + 2).rem_euclid(4), 0);
        assert_eq!((3usize + 2).rem_euclid(4), 1);
    }
}