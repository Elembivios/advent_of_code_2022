use std::{str::FromStr, fmt};
use crate::utils::point::*;

pub struct RopeBridge {
    commands: Vec<(Direction, usize)>
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDirectionError;
impl FromStr for Direction {
    type Err = ParseDirectionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::N),
            "D" => Ok(Direction::S),
            "L" => Ok(Direction::W),
            "R" => Ok(Direction::E),
            _ => Err(ParseDirectionError)
        }
    }
}

impl Coord<isize> {
    fn step(&mut self, tail: &mut Coord<isize>, direction: &Direction) -> bool {
        let dir_vec: Coord<isize> = direction.into();
        let axis: Axis = direction.into();
        *self += dir_vec.clone();
        let diff = match direction {
            Direction::E => { tail.x - self.x },
            Direction::W => { self.x - tail.x },
            Direction::N => { self.y - tail.y },
            Direction::S => { tail.y - self.y },
        };
        // println!("Diff: {:?}", diff);
        // Check if the tail needs to move
        if diff > 1 {
            // Changes x or y by dir_vec
            *tail.get_mut(&axis) += dir_vec.get(&axis);
            // Sets x or y to be the same as self (head)
            let other_axis = axis.other();
            *tail.get_mut(&other_axis) = *self.get(&other_axis);
            true
        } else {
            false
        }
    }
}

impl crate::Advent for RopeBridge {
    fn new(data: &str) -> Self {
        let commands: Vec<(Direction, usize)> = data
            .lines()
            .map(|l| {
                let (dir, num) = l.split_once(" ").unwrap();
                (Direction::from_str(dir).unwrap(), num.parse().unwrap())                
            })
            .collect();
        RopeBridge { commands }
    }

    fn part_01(&self) -> String {
        let mut rope = Rope { 
            head: Coord::new(0, 0),
            tail: vec![Coord::new(0, 0)]
        };
        let mut end_positions: Vec<Coord<isize>> = vec![Coord::new(0, 0)];

        for (dir, steps) in &self.commands {            
            let mut new_end_positions = rope.exec_command(dir, *steps);
            end_positions.append(&mut new_end_positions);
        }

        end_positions.sort_unstable();
        end_positions.dedup();
        end_positions.len().to_string()
    }

    fn part_02(&self) -> String {
        let mut rope = Rope { 
            head: Coord::new(0, 0),
            tail: (0..10).map(|_| Coord::new(0, 0)).into_iter().collect()
        };
        let mut end_positions: Vec<Coord<isize>> = vec![Coord::new(0, 0)];

        for (dir, steps) in &self.commands {            
            let mut new_end_positions = rope.exec_command(dir, *steps);
            end_positions.append(&mut new_end_positions);        
        }

        end_positions.sort_unstable();
        end_positions.dedup();
        end_positions.len().to_string()    
    }
}


struct Rope {
    head: Coord<isize>,
    tail: Vec<Coord<isize>>    
}

impl Rope {
    fn exec_command(&mut self, dir: &Direction, steps: usize) -> Vec<Coord<isize>> {
        let mut end_positions: Vec<Coord<isize>> = vec![];
        let last_index = self.tail.len() - 1;
        for x in 0..steps {        
            let mut current = &mut self.head;
            for (i, part) in self.tail.iter_mut().enumerate() {
                
                let part_moved = current.step(part, dir);
                if !part_moved {
                    break;
                }
                current = part;
                if i == last_index {
                    end_positions.push(current.clone());
                }
            }
        }
        end_positions
    }
}

impl fmt::Debug for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min: isize = -7;
        let max: isize = 7;
        write!(f, "\n")?;
        for y in min..=max {
            let s: String = (min..=max).map(|x| {
                let current: Coord<isize> = Coord::new(x, y);
                if self.head == current {
                    return 'H';
                }
                if let Some(pos) = self.tail.iter().position(|coord| {
                    *coord == current
                }) {
                    return char::from_digit(pos as u32, 10).unwrap();
                }
                if current.x == 0 && current.y == 0 {
                    return 'X';
                }
                '.'
            }).collect();
            write!(f, "{}\n", s)?;
        }
        write!(f, "\n")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_east() {
        let mut rope = Rope {
            head: Coord::new(0, 0),
            tail: vec![Coord::new(0, 0)]
        };    
        let tail_positions = rope.exec_command(&Direction::E, 4); 
        assert_eq!(tail_positions, vec![
            Coord::new(-1,0),
            Coord::new(-2,0),
            Coord::new(-3,0),
        ]);
        assert_eq!(rope.head, Coord::new(-4, 0));
        assert_eq!(rope.tail[0], Coord::new(-3, 0));
        

        let mut rope = Rope {
            head: Coord::new(0, 0),
            tail: vec![Coord::new(0, -1)]
        };    
        let tail_positions = rope.exec_command(&Direction::E, 4); 
        assert_eq!(tail_positions, vec![
            Coord::new(-1,0),
            Coord::new(-2,0),
            Coord::new(-3,0),
        ]);
        assert_eq!(rope.head, Coord::new(-4, 0));
        assert_eq!(rope.tail[0], Coord::new(-3, 0));
    }

    #[test]
    fn test_right() {
        let mut rope = Rope {
            head: Coord::new(0, 0),
            tail: vec![
                Coord::new(0, 0),
                Coord::new(0, 0)
            ]
        };    
        let tail_positions = rope.exec_command(&Direction::W, 4); 
        
    }
}