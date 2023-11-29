
use crate::utils::point::{Point, Coord, Direction};
use core::fmt;

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

impl BlizardBasin {
    fn sort_blizzards(&self, &mut mut blizzards: &mut Vec<Blizzard>) {
        blizzards.sort_unstable_by(|a, b| {
            a.coord.cmp(&b.coord)
        });        
    }
}

impl fmt::Display for BlizardBasin
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        let mut blizzards = self.blizzards.clone();
        self.sort_blizzards(&mut blizzards);        
        
        let mut blizzards_it = blizzards.iter();

        for y in 0..self.height {            
            for x in 0..self.width {
                let c = Coord::new(x, y);            
                let current_coord_blizards: Vec<&Blizzard> = blizzards_it.take_while(|b| {
                    b.coord == c
                }).collect();
                match current_coord_blizards.len() {
                    0 => write!(f, ".")?,
                    1 => write!(f, "{}", current_coord_blizards[0].value)?,
                    n => write!(f, "{}", n)?
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }    
}