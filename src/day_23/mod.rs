use crate::utils::point::{Coord, Point, Direction, DIRECTIONS};
use crate::utils::wait_user_input;
use std::collections::HashMap;


type Elf = Point<i32, Direction>;
type C = Coord<i32>;
pub struct UnstableDiffusion {
    elfs: Vec<Elf>
}

impl crate::Advent for UnstableDiffusion {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let height = data.lines().count() as i32;
        let elfs: Vec<Elf> = data.lines().enumerate().map(|(y, l)| {
            l.chars().enumerate().filter_map(|(x, c)| {
                match c {
                    '#' => Some(Point::new(x as i32, height - 1 - y as i32, Direction::E)),
                    _ => None
                }
            }).collect::<Vec<_>>()
        }).flatten()
        .collect();    
        Self { elfs }
    }

    fn part_01(&self) -> String {
        const DIRECTION_ORDER: [Direction; 4] = [Direction::N, Direction::S, Direction::W, Direction::E];
        let mut elfs = self.elfs.clone();        
        for i in 0..10 {
            println!("{} -- Elfs: {:?}", i, elfs);
            UnstableDiffusion::display_elfs(&elfs);
            let mut moves: HashMap<usize, Direction> = HashMap::new();
            'check_moves: for (elf_i, elf) in elfs.iter().enumerate() {
                let neigbour_elfs: Vec<Option<&Elf>> = DIRECTIONS
                    .iter()
                    .map(|direction| {
                        let nc = elf.coord.get_neighbour(direction);
                        elfs.iter().filter(|p| p.coord == nc).next()
                    }).collect();                
    
                // No need to move if all spaces are empty 
                if neigbour_elfs.iter().all(|n| n.is_none()) {
                    continue 'check_moves;
                }        
                let last_dir = elf.value;
                let dir_pos = DIRECTION_ORDER.iter().position(|d| *d == last_dir).unwrap();
    
                'search_direction: for new_dir in DIRECTION_ORDER.iter().cycle().skip(dir_pos + 1).take(DIRECTION_ORDER.len()) {
                    let facing_spaces: [Option<&Elf>; 3] = match new_dir {
                        Direction::N => {[neigbour_elfs[7], neigbour_elfs[0], neigbour_elfs[1]]},
                        Direction::S => {[neigbour_elfs[3], neigbour_elfs[4], neigbour_elfs[5]]},
                        Direction::E => {[neigbour_elfs[1], neigbour_elfs[2], neigbour_elfs[3]]},
                        Direction::W => {[neigbour_elfs[5], neigbour_elfs[6], neigbour_elfs[7]]},
                        _ => panic!("Invalid dir {:?}", last_dir)
                    };
                    if facing_spaces.iter().all(|s| s.is_none()) {
                        moves.insert(elf_i, *new_dir);                        
                        break 'search_direction;
                    }
                }
            }

            let mut moves_to: HashMap<usize, C> = HashMap::with_capacity(moves.len());
            for (elf_i, new_dir) in moves.iter() {
                let elf = elfs.get_mut(*elf_i).unwrap();
                let new_coord = elf.coord.get_neighbour(new_dir);
                moves_to.insert(*elf_i, new_coord);
                elf.value = *new_dir;
            }

            let mut target_count: HashMap<C, usize> = HashMap::with_capacity(moves_to.len());
            for (_elf_i, target_coord) in moves_to.iter() {
                target_count.entry(*target_coord).and_modify(|counter| *counter += 1).or_insert(1);
            }

            let duplicated_targets: Vec<C> = target_count
                .into_iter()
                .filter_map(|(target_coord, counter)| {
                    if counter > 1 {
                        Some(target_coord)
                    } else {
                        None
                    }
                }).collect();

            moves_to
                .into_iter()
                .filter(|(_elf_i, target_coord)| {
                    !duplicated_targets.contains(target_coord)
                }).for_each(|(elf_i, target_coord)| {
                    let elf = elfs.get_mut(elf_i).unwrap();
                    elf.coord = target_coord;
                });
        }        
        1.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}

impl UnstableDiffusion {
    fn display_elfs(elfs: &Vec<Elf>) {
        let mut max_x = i32::MIN;
        let mut min_x = 0;        
        let mut max_y = i32::MIN;
        let mut min_y = 0;
        for elf in elfs {
            if elf.coord.x < min_x {
                min_x = elf.coord.x;
            } 
            if elf.coord.x > max_x {
                max_x = elf.coord.x;
            }
            if elf.coord.y < min_y {
                min_y = elf.coord.y;
            } 
            if elf.coord.y > max_y {
                max_y = elf.coord.y;
            }            
        }
        println!("X: {} -> {}, Y: {} -> {}", min_x, max_x, min_y, max_y);

        for y in (min_y..=max_y).rev() {
            let mut row: String = String::new();
            for x in min_x..=max_x {                
                if elfs.iter().any(|p| p.coord.x == x && p.coord.y == y) {
                    row.push('#');
                } else {
                    row.push('.');
                }
                
            }
            println!("{}", row);
        }
    }
}