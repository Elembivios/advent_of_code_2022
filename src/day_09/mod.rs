use std::{str::FromStr, fmt};
use crate::utils::{point::*, wait_user_input};

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
    fn move_to(&mut self, direction: &Direction) {
        let dir_vec: Coord<isize> = direction.into();
        *self += dir_vec;
    }

    fn tail_need_to_move(&self, tail: &Coord<isize>) -> Option<Direction> {
        let x_diff = self.x - tail.x;
        let y_diff = self.y - tail.y;
        match (x_diff, y_diff) {
            ( 2,  2) => Some(Direction::NE),
            ( 2, -2) => Some(Direction::SE),
            (-2,  2) => Some(Direction::NW),
            (-2, -2) => Some(Direction::SW),

            ( 2,  _) => Some(Direction::E),
            (-2,  _) => Some(Direction::W),
            ( _,  2) => Some(Direction::N),
            ( _, -2) => Some(Direction::S),
            _ => None
        }
    }

    fn update_tail(&self, tail: &mut Coord<isize>, direction: &Direction) {
        let axes: Vec<Axis> = direction.affected_axes();
        let dir_vec: Coord<isize> = direction.into();
        for axis in &axes {            
            *tail.get_mut(&axis) += dir_vec.get(axis);            
        }
        if axes.len() == 1 {
            let other_axis = axes[0].other();
            *tail.get_mut(&other_axis) = *self.get(&other_axis);
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
            tail: (0..9).map(|_| Coord::new(0, 0)).into_iter().collect()
        };
        let mut end_positions: Vec<Coord<isize>> = vec![Coord::new(0, 0)];

        for (dir, steps) in &self.commands {            
            let mut new_end_positions = rope.exec_command(dir, *steps);
            end_positions.append(&mut new_end_positions);        
            // rope.print_and_wait();
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
    #[allow(dead_code)]
    fn print_and_wait(&self) {
        print!("{esc}c", esc = 27 as char);
        println!("{:?}", self);    
        wait_user_input();     
    }

    fn exec_command(&mut self, dir: &Direction, steps: usize) -> Vec<Coord<isize>> {
        let mut end_positions: Vec<Coord<isize>> = vec![];
        let last_index = self.tail.len() - 1;
        for _step in 0..steps {

            self.head.move_to(&dir);
            let mut current = &mut self.head;
            for (i, part) in self.tail.iter_mut().enumerate() {                
                let part_needs_to_move = current.tail_need_to_move(part);

                let Some(direction) = part_needs_to_move else {
                    break;
                };

                current.update_tail(part, &direction);
                current = part;
                if i == last_index {
                    end_positions.push(current.clone())
                }                
            }
                     
        }
        end_positions
    }
}

impl fmt::Debug for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min: isize = -15;
        let max: isize = 15;
        write!(f, "\n")?;
        for y in (min..=max).rev() {
            let s: String = (min..=max).map(|x| {
                let current: Coord<isize> = Coord::new(x, y);
                if self.head == current {
                    return 'H';
                }
                if let Some(pos) = self.tail.iter().position(|coord| {
                    *coord == current
                }) {
                    return char::from_digit((pos + 1) as u32, 10).unwrap_or('#');
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
    fn visual_test_small_example() {            
        let mut rope = Rope {
            head: Coord::new(0, 0),
            tail: (0..9).map(|_| Coord::new(0, 0)).into_iter().collect()
        };    
        let commands: Vec<(Direction, usize)> = vec![
            (Direction::E, 4),
            (Direction::N, 4),
            (Direction::W, 3),
            (Direction::S, 1),
            (Direction::E, 4),
            (Direction::S, 1),
            (Direction::W, 5),
            (Direction::E, 2),
        ];
        for (dir, steps) in &commands {
            rope.exec_command(dir, *steps);
            rope.print_and_wait();
        }
    }

    #[test]
    fn test_big_example() {            
        let mut rope = Rope {
            head: Coord::new(0, 0),
            tail: (0..9).map(|_| Coord::new(0, 0)).into_iter().collect()
        };    
        let commands: Vec<(Direction, usize)> = vec![
            (Direction::E, 5),
            (Direction::N, 8),
            (Direction::W, 8),
            (Direction::S, 3),
            (Direction::E, 17),
            (Direction::S, 10),
            (Direction::W, 25),
            (Direction::N, 20),
        ];
        let mut tail_positions: Vec<Coord<isize>> = vec![Coord::new(0,0)];
        for (dir, steps) in &commands {
            let mut new_tail_pos = rope.exec_command(dir, *steps);            
            tail_positions.append(&mut new_tail_pos);
        }
        assert_eq!(tail_positions.len(), 36);
        let tail_pos_rope = Rope {
            head: tail_positions[0].clone(),
            tail: tail_positions.into_iter().skip(1).collect()
        };
        
        tail_pos_rope.print_and_wait();
    }
}

