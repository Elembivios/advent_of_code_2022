pub struct TreeTopTreeHouse {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize
}

impl TreeTopTreeHouse {
    fn visible_from_left(&self) -> Vec<(usize, usize)> {
        let visible_coords: Vec<(usize, usize)> = self
        .grid.iter().enumerate().map(|(y, row)| {
            let mut current_max: i8 = -1;
            row.iter().enumerate().filter_map(|(x, val)| {
                if *val as i8> current_max {
                    current_max = *val as i8;
                    Some((x, y))
                } else {
                    None
                }
            }).collect::<Vec<(usize, usize)>>()
        }).flatten().collect();
        visible_coords
    }

    fn visible_from_right(&self) -> Vec<(usize, usize)> {
        let visible_coords: Vec<(usize, usize)> = self.grid
        .iter().enumerate().rev().inspect(|e| println!("e: {:?}", e)).map(|(y, row)| {
            let mut current_max: i8 = -1;
            row.iter().enumerate().rev().filter_map(|(x, val)| {
                if *val as i8 > current_max {
                    current_max = *val as i8;
                    Some((x, y))
                } else {
                    None
                }
            }).collect::<Vec<(usize, usize)>>()
        }).flatten().collect();
        visible_coords
    }

    fn visible_from_top(&self) -> Vec<(usize, usize)> {
        let mut visible_coords: Vec<(usize, usize)> = vec![];
        for x in 0..self.width {
            let mut current_max: i8 = -1;
            for y in 0..self.height {
                let val = self.grid[y][x] as i8;
                if val > current_max {
                    current_max = val;
                    visible_coords.push((x, y))
                }
            }
        }
        visible_coords
    }

    fn visible_from_bottom(&self) -> Vec<(usize, usize)> {
        let mut visible_coords: Vec<(usize, usize)> = vec![];
        for x in 0..self.width {
            let mut current_max: i8 = -1;
            for y in self.height..0 {
                let val = self.grid[y][x] as i8;
                if val > current_max {
                    current_max = val;
                    visible_coords.push((x, y))
                }
            }
        }
        visible_coords
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
        let left = self.visible_from_left();        
        let right = self.visible_from_right();        
        let top = self.visible_from_top();
        let bottom = self.visible_from_bottom();
        println!("Left: {:?}", left);        
        println!("Right: {:?}", right);
        println!("Top: {:?}", top);        
        println!("Botto: {:?}", bottom);        
        let mut visible: Vec<(usize, usize)> = left.into_iter().chain(right).chain(top).chain(bottom).collect();
        visible.sort_unstable();
        // visible.sort_by(|a, b| {
        //     let lhs = a.0.cmp(&b.0);
        //     match lhs {
        //         Ordering::Equal => {
        //             a.1.cmp(&b.1)
        //         },
        //         _ => lhs
        //     }
        // });        
        // visible.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
        visible.dedup();
        println!("All {:?}", visible);
        visible.iter().count().to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}


