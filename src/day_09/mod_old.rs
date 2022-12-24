use std::str::FromStr;
use crate::utils::point::*;

pub struct RopeBridge {
    commands: Vec<Command>
}


enum Command
{ 
    U(u8),
    D(u8), 
    L(u8), 
    R(u8) 
}

impl Command {
    fn track_tail(&self, head: &mut Coord<isize>, tail: &mut Coord<isize>) -> Vec<Coord<isize>> {
        match self {
            Command::L(steps) => {
                let start = match head.x - tail.x {
                    0 => 1,
                    1 => 2,
                    _ => 0
                };
                let tail_positions: Vec<Coord<isize>> = (start..*steps).map(|x| {
                    Coord::new(head.x - x as isize, head.y)
                }).collect();
                if let Some(last_pos) = tail_positions.last() {
                    tail.x = last_pos.x;
                    tail.y = last_pos.y;
                }
                head.x -= *steps as isize;
                tail_positions
            },
            Command::R(steps) => {
                let start = match tail.x - head.x {
                    0 => 1,
                    1 => 2,
                    _ => 0
                };
                let tail_positions: Vec<Coord<isize>> = (start..*steps).map(|x| {
                    Coord::new(head.x + x as isize, head.y)
                }).collect();
                if let Some(last_pos) = tail_positions.last() {
                    tail.x = last_pos.x;
                    tail.y = last_pos.y;
                }
                head.x += *steps as isize;
                tail_positions
            },
            Command::U(steps) => {
                let start = match tail.y - head.y {
                    0 => 1,
                    1 => 2,
                    _ => 0
                };
                let tail_positions: Vec<Coord<isize>> = (start..*steps).map(|y| {
                    Coord::new(head.x, head.y + y as isize)
                }).collect();
                if let Some(last_pos) = tail_positions.last() {
                    tail.x = last_pos.x;
                    tail.y = last_pos.y;
                }
                head.y += *steps as isize;
                tail_positions
            },
            Command::D(steps) => {
                let start = match head.y - tail.y {
                    0 => 1,
                    1 => 2,
                    _ => 0
                };
                let tail_positions: Vec<Coord<isize>> = (start..*steps).map(|y| {
                    Coord::new(head.x, head.y - y as isize)
                }).collect();
                if let Some(last_pos) = tail_positions.last() {
                    tail.x = last_pos.x;
                    tail.y = last_pos.y;
                }
                head.y -= *steps as isize;
                tail_positions
            },
            
        }
    }
}

impl Coord<isize> {
    fn step_one(&mut self, command: &Command, tail: &mut Coord<isize>) -> Vec<Coord<isize>> {
        let original_position = self.clone();
        match command {
            Command::L(_) => self.x -= 1,
            Command::R(_) => self.x += 1,
            Command::U(_) => self.y += 1,
            Command::D(_) => self.y -= 1
        };
        


        vec![]
    }
}

struct Rope {
    head: Coord<isize>,
    tail: Vec<Coord<isize>>,
}

impl Rope {
    fn move_by(&mut self, command: &Command) -> Vec<Coord<isize>> {
        for knot in self.tail.iter_mut() {
            command.track_tail(&mut self.head, knot);
        }
        vec![]
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCommandError;
impl FromStr for Command {
    type Err = ParseCommandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, v) = s.split_once(" ").ok_or(ParseCommandError)?;
        let val: u8 = v.parse().map_err(|_| ParseCommandError)?;
        match c {
            "U" => Ok(Command::U(val)),
            "D" => Ok(Command::D(val)),
            "L" => Ok(Command::L(val)),
            "R" => Ok(Command::R(val)),
            _ => Err(ParseCommandError)
        }
    }
}

impl crate::Advent for RopeBridge {
    fn new(data: &str) -> Self {
        let commands: Vec<Command> = data
            .lines()
            .map(|l| Command::from_str(l).unwrap())
            .collect();
        RopeBridge { commands }
    }

    fn part_01(&self) -> String {
        let mut head: Coord<isize> = Coord::new(0, 0);
        let mut tail: Coord<isize> = Coord::new(0, 0);
        let mut tail_positions: Vec<Coord<isize>> = vec![Coord::new(0, 0)];

        for command in &self.commands {            
            let mut new_tail_positions = command.track_tail(&mut head, &mut tail);
            tail_positions.append(&mut new_tail_positions);
        }

        tail_positions.sort_unstable();
        tail_positions.dedup();
        tail_positions.len().to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left() {
        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(0, 0);
        let command = Command::L(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(-1,0),
            Coord::new(-2,0),
            Coord::new(-3,0),
        ]);
        assert_eq!(head, Coord::new(-4, 0));
        assert_eq!(tail, Coord::new(-3, 0));

        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(0, -1);
        let command = Command::L(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(-1,0),
            Coord::new(-2,0),
            Coord::new(-3,0),
        ]);
        assert_eq!(head, Coord::new(-4, 0));
        assert_eq!(tail, Coord::new(-3, 0));

        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(1, -1);
        let command = Command::L(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(0,0),
            Coord::new(-1,0),
            Coord::new(-2,0),
            Coord::new(-3,0),
        ]);
        assert_eq!(head, Coord::new(-4, 0));
        assert_eq!(tail, Coord::new(-3, 0));

        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(1, 0);
        let command = Command::L(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(0,0),
            Coord::new(-1,0),
            Coord::new(-2,0),
            Coord::new(-3,0),
        ]);
        assert_eq!(head, Coord::new(-4, 0));
        assert_eq!(tail, Coord::new(-3, 0));

        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(1, 1);
        let command = Command::L(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(0,0),
            Coord::new(-1,0),
            Coord::new(-2,0),
            Coord::new(-3,0),
        ]);
        assert_eq!(head, Coord::new(-4, 0));
        assert_eq!(tail, Coord::new(-3, 0));

        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(0, 1);
        let command = Command::L(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(-1,0),
            Coord::new(-2,0),
            Coord::new(-3,0),
        ]);
        assert_eq!(head, Coord::new(-4, 0));
        assert_eq!(tail, Coord::new(-3, 0));

        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(-1, 1);
        let command = Command::L(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(-2,0),
            Coord::new(-3,0),
        ]);
        assert_eq!(head, Coord::new(-4, 0));
        assert_eq!(tail, Coord::new(-3, 0));

        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(-1, 0);
        let command = Command::L(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(-2,0),
            Coord::new(-3,0),
        ]);
        assert_eq!(head, Coord::new(-4, 0));
        assert_eq!(tail, Coord::new(-3, 0));
    }

    #[test]
    fn test_right() {
        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(-1, 0);
        let command = Command::R(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(0, 0),
            Coord::new(1, 0),
            Coord::new(2, 0),
            Coord::new(3, 0),
        ]);
        assert_eq!(head, Coord::new(4, 0));
        assert_eq!(tail, Coord::new(3, 0));

        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(-1, -1);
        let command = Command::R(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(0, 0),
            Coord::new(1, 0),
            Coord::new(2, 0),
            Coord::new(3, 0),
        ]);
        assert_eq!(head, Coord::new(4, 0));
        assert_eq!(tail, Coord::new(3, 0));

        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(1, 1);
        let command = Command::R(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(2, 0),
            Coord::new(3, 0),
        ]);
        assert_eq!(head, Coord::new(4, 0));
        assert_eq!(tail, Coord::new(3, 0));
    }

    #[test]
    fn test_up() {
        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(0, -1);
        let command = Command::U(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(0, 0),
            Coord::new(0, 1),
            Coord::new(0, 2),
            Coord::new(0, 3),
        ]);
        assert_eq!(head, Coord::new(0, 4));
        assert_eq!(tail, Coord::new(0, 3));
    }

    #[test]
    fn test_down() {
        let mut head = Coord::new(0, 0);
        let mut tail = Coord::new(0, 1);
        let command = Command::D(4);
        let tail_positions = command.track_tail(&mut head, &mut tail);
        assert_eq!(tail_positions, vec![
            Coord::new(0, 0),
            Coord::new(0, -1),
            Coord::new(0, -2),
            Coord::new(0, -3),
        ]);
        assert_eq!(head, Coord::new(0, -4));
        assert_eq!(tail, Coord::new(0, -3));
    }
}