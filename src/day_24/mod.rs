
use crate::utils::point::{Point, Coord, Direction};

type Blizzard = Point<usize, Direction>;
pub struct BlizardBasin {
    height: usize,
    width: usize,
    blizzards: Vec<Blizzard>
}

impl crate::Advent for BlizardBasin {
    fn new(data: &str) -> Self
        where 
            Self: Sized {        
        let height = data.len() - 2;
        let width = data.lines().next().unwrap().len() - 2;

        let blizzards: Vec<Blizzard> = data
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        let dir = match c {
                            '<' => Some(Direction::W),
                            '>' => Some(Direction::E),
                            '^' => Some(Direction::N),
                            'v' => Some(Direction::S),
                            _ => None
                        };
                        match dir {
                            Some(dir) => Some(Point::new(x - 1, y - 1, dir)),
                            None => None
                        }
                    }).collect::<Vec<_>>()
            }).flatten()
            .collect();
        Self {
            height,
            width,
            blizzards
        }
    }

    fn part_01(&self) -> String {
        
        1.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}