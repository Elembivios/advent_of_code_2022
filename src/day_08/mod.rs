use std::collections::HashMap;

pub struct TreeTopTreeHouse {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

impl TreeTopTreeHouse {
    fn visible_from(&self, direction: &Direction) -> Vec<(usize, usize)> {
        let visible: Vec<(usize, usize)> = match direction {
            Direction::West => {
                (0..self.height).map(|y| {
                    let mut max_height: i8 = -1;
                    (0..self.width).filter_map(|x| {
                        let val = self.grid[y][x] as i8;
                        if val > max_height {
                            max_height = val;
                            return Some((x, y));
                        }
                        None
                    }).collect::<Vec<(usize, usize)>>()
                }).flatten().collect()
            },
            Direction::East => {
                (0..self.height).map(|y| {
                    let mut max_height: i8 = -1;
                    (0..self.width).rev().filter_map(|x| {
                        let val = self.grid[y][x] as i8;
                        if val > max_height {
                            max_height = val;
                            return Some((x, y));
                        }
                        None
                    }).collect::<Vec<(usize, usize)>>()
                }).flatten().collect()
            },
            Direction::North => {
                (0..self.width).map(|x| {
                    let mut max_height: i8 = -1;
                    (0..self.height).filter_map(|y| {
                        let val = self.grid[y][x] as i8;
                        if val > max_height {
                            max_height = val;
                            return Some((x, y));
                        }
                        None
                    }).collect::<Vec<(usize, usize)>>()
                }).flatten().collect()
            },            
            Direction::South => {
                (0..self.width).map(|x| {
                    let mut max_height: i8 = -1;
                    (0..self.height).rev().filter_map(|y| {
                        let val = self.grid[y][x] as i8;
                        if val > max_height {
                            max_height = val;
                            return Some((x, y));
                        }
                        None
                    }).collect::<Vec<(usize, usize)>>()
                }).flatten().collect()
            },            
        };
        visible        
    }

    fn scenic_scores_in_direction(&self, direction: &Direction) -> Vec<Vec<usize>> {
        match direction {
            Direction::West => {
                (0..self.height).map(|y| {
                    // A map of <tree_size, position> of last seen of it's size
                    let mut scenic_scores: HashMap<u8, usize> = (0..10).map(|k| (k, 0)).collect();
                    (0..self.width).map(|x| {
                        let val = self.grid[y][x];
                        let scenic_score = x - scenic_scores[&val];

                        // Reset all smaller trees position to 0
                        scenic_scores
                            .iter_mut()
                            .filter(|(k, _v)| **k <= val)
                            .for_each(|(_k, v)| *v = x);                    
                        scenic_score
                    }).collect::<Vec<usize>>()
                }).collect()
            },
            Direction::East => {
                (0..self.height).map(|y| {
                    // A map of <tree_size, position> of last seen of it's size
                    let mut scenic_scores: HashMap<u8, usize> = (0..10).map(|k| (k, self.width - 1)).collect();
                    (0..self.width).rev().map(|x| {
                        let val = self.grid[y][x];
                        let scenic_score = scenic_scores[&val] - x;

                        // Reset all smaller trees position to 0
                        scenic_scores
                            .iter_mut()
                            .filter(|(k, _v)| **k <= val)
                            .for_each(|(_k, v)| *v = x);                    
                        scenic_score
                    }).collect::<Vec<usize>>()
                }).collect()
            },
            Direction::North => {
                (0..self.width).map(|x| {
                    // A map of <tree_size, position> of last seen of it's size
                    let mut scenic_scores: HashMap<u8, usize> = (0..10).map(|k| (k, 0)).collect();
                    (0..self.height).map(|y| {
                        let val = self.grid[y][x];
                        let scenic_score = y - scenic_scores[&val];

                        // Reset all smaller trees position to 0
                        scenic_scores
                            .iter_mut()
                            .filter(|(k, _v)| **k <= val)
                            .for_each(|(_k, v)| *v = y);                    
                        scenic_score
                    }).collect::<Vec<usize>>()
                }).collect()
            },            
            Direction::South => {
                (0..self.width).map(|x| {
                    // A map of <tree_size, position> of last seen of it's size
                    let mut scenic_scores: HashMap<u8, usize> = (0..10).map(|k| (k, self.height - 1)).collect();
                    (0..self.height).rev().map(|y| {
                        let val = self.grid[y][x];
                        let scenic_score = scenic_scores[&val] - y;

                        // Reset all smaller trees position to 0
                        scenic_scores
                            .iter_mut()
                            .filter(|(k, _v)| **k <= val)
                            .for_each(|(_k, v)| *v = y);                    
                        scenic_score
                    }).collect::<Vec<usize>>()
                }).collect()
            },            
        }
    }
}

impl crate::Advent for TreeTopTreeHouse {
    fn new(data: &str) -> TreeTopTreeHouse {
        let grid: Vec<Vec<u8>> = data.lines().map(|l| {
            l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
        }).collect();
        let width = grid[0].len();
        let height = grid.len();
        TreeTopTreeHouse { grid, width, height }
    }

    fn part_01(&self) -> String {
        
        let west = self.visible_from(&Direction::West);
        let east = self.visible_from(&Direction::East);
        let north = self.visible_from(&Direction::North);
        let south = self.visible_from(&Direction::South);
        let mut visible: Vec<_> = west.into_iter().chain(east).chain(north).chain(south).collect();
        visible.sort_unstable();
        visible.dedup();

        visible.iter().count().to_string()
    }

    fn part_02(&self) -> String {
        let west = self.scenic_scores_in_direction(&Direction::West);        
        let east = self.scenic_scores_in_direction(&Direction::East);
        let north = self.scenic_scores_in_direction(&Direction::North);
        let south = self.scenic_scores_in_direction(&Direction::South);

        let max_scenic_score = west.into_iter().flatten()
            .zip(
                east.into_iter().map(|row| row.into_iter().rev()).flatten()
            )
            .zip(
                transpose(north).into_iter().flatten()        
            )
            .zip(
                transpose(south).into_iter().rev().flatten()
            )
            .map(|(((a, b), c), d)| a * b * c * d)
            .max();
        max_scenic_score.unwrap().to_string()
    }
}


fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = if let Some(first) = original.get(0) {
        first.len()
    } else {
        0
    };
    let mut iters: Vec<_> = original
        .into_iter()
        .map(|n| n.into_iter()).collect();
    (0..len).map(|_| {
        iters.iter_mut().map(|n| {
            n.next().unwrap() // Considers all cols are of same length
        }).collect::<Vec<T>>()
    }).collect()
}


#[cfg(test)]
mod tests { 
    #[test]
    fn test_range_ref() {
        assert!(!(6..3).contains(&3));
        for num in (3..6).rev() {
            println!("Num: {:?}", num);
        }

        assert!((0..6).contains(&0));
        assert!((0..7).rev().any(|n| n==0))
    }


}

