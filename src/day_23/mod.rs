use crate::utils::point::{Coord, Direction, DIRECTIONS};
use std::collections::HashMap;

type C = Coord<i32>;
pub struct UnstableDiffusion {
    elfs: Vec<C>
}

impl crate::Advent for UnstableDiffusion {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let height = data.lines().count() as i32;
        let elfs: Vec<C> = data.lines().enumerate().map(|(y, l)| {
            l.chars().enumerate().filter_map(|(x, c)| {
                match c {
                    '#' => Some(C::new(x as i32, height - 1 - y as i32)),
                    _ => None
                }
            }).collect::<Vec<_>>()
        }).flatten()
        .collect();    
        Self { elfs }
    }

    fn part_01(&self) -> String {
        let (_rounds, elfs) = self.move_elfs(Some(10));
        UnstableDiffusion::count_empty(&elfs).to_string()
    }

    fn part_02(&self) -> String {
        let (rounds, _elfs) = self.move_elfs(None);
        (rounds + 1).to_string()
    }
}

impl UnstableDiffusion {
    #[allow(dead_code)]
    fn display_elfs(elfs: &Vec<C>) {
        let mut max_x = i32::MIN;
        let mut min_x = 0;        
        let mut max_y = i32::MIN;
        let mut min_y = 0;
        for coord in elfs {
            if coord.x < min_x {
                min_x = coord.x;
            } 
            if coord.x > max_x {
                max_x = coord.x;
            }
            if coord.y < min_y {
                min_y = coord.y;
            } 
            if coord.y > max_y {
                max_y = coord.y;
            }            
        }
        println!("X: {} -> {}, Y: {} -> {}", min_x, max_x, min_y, max_y);

        for y in (min_y..=max_y).rev() {
            let mut row: String = String::new();
            for x in min_x..=max_x {                
                if elfs.iter().any(|c: &Coord<i32>| c.x == x && c.y == y) {
                    row.push('#');
                } else {
                    row.push('.');
                }
                
            }
            println!("{}", row);
        }
    }

    fn count_empty(elfs: &Vec<C>) -> usize {
        let mut max_x = i32::MIN;
        let mut min_x = i32::MAX;        
        let mut max_y = i32::MIN;
        let mut min_y = i32::MAX;
        for coord in elfs {
            if coord.x < min_x {
                min_x = coord.x;
            } 
            if coord.x > max_x {
                max_x = coord.x;
            }
            if coord.y < min_y {
                min_y = coord.y;
            } 
            if coord.y > max_y {
                max_y = coord.y;
            }            
        }
        let mut empty_counter = 0;
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {                
                if !elfs.iter().any(|c| c.x == x && c.y == y) {
                    empty_counter += 1;
                }                
            }
        }

        empty_counter
    }

    fn move_elfs(&self, round_limit: Option<usize>) -> (usize, Vec<C>) {
        const DIRECTION_ORDER: [Direction; 4] = [Direction::N, Direction::S, Direction::W, Direction::E];
        let mut elfs = self.elfs.clone();
        let mut i = 0;
        'move_elfs: loop {
            match round_limit {
                Some(rl) => if i >= rl { break; },
                None => {}
            }
            let dir_pos = i % DIRECTION_ORDER.len();
            // println!("{} -- Elfs: {:?}", i, elfs);
            // UnstableDiffusion::display_elfs(&elfs);
            let mut moves: HashMap<usize, Direction> = HashMap::new();
            'check_moves: for (elf_i, elf) in elfs.iter().enumerate() {
                let neigbour_elfs: Vec<Option<&C>> = DIRECTIONS
                    .iter()
                    .map(|direction| {
                        let nc = elf.get_neighbour(direction);
                        elfs.iter().filter(|c| **c == nc).next()
                    }).collect();                
    
                // No need to move if all spaces are empty 
                if neigbour_elfs.iter().all(|n| n.is_none()) {
                    continue 'check_moves;
                }
                'search_direction: for new_dir in DIRECTION_ORDER.iter().cycle().skip(dir_pos).take(DIRECTION_ORDER.len()) {
                    let facing_spaces: [Option<&C>; 3] = match new_dir {
                        Direction::N => {[neigbour_elfs[7], neigbour_elfs[0], neigbour_elfs[1]]},
                        Direction::S => {[neigbour_elfs[3], neigbour_elfs[4], neigbour_elfs[5]]},
                        Direction::E => {[neigbour_elfs[1], neigbour_elfs[2], neigbour_elfs[3]]},
                        Direction::W => {[neigbour_elfs[5], neigbour_elfs[6], neigbour_elfs[7]]},
                        _ => panic!("Invalid dir {:?}", dir_pos)
                    };
                    if facing_spaces.iter().all(|s| s.is_none()) {
                        moves.insert(elf_i, *new_dir);                        
                        break 'search_direction;
                    }
                }
            }

            if moves.len() == 0 {
                break 'move_elfs;
            }

            let mut moves_to: HashMap<usize, C> = HashMap::with_capacity(moves.len());
            for (elf_i, new_dir) in moves.iter() {
                let elf = elfs.get_mut(*elf_i).unwrap();
                let new_coord = elf.get_neighbour(new_dir);
                moves_to.insert(*elf_i, new_coord);
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
                    *elf = target_coord;
                });

            i += 1;
        }
        (i, elfs)
    }
}