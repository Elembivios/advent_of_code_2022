// 3284 -- too high

use std::{collections::{VecDeque, HashSet}, ops::RangeBounds};

type Coord = [i8; 3];


fn get_neighbour_coords(c: &Coord) -> Vec<Coord> {
    let mut neighbours: Vec<Coord> = vec![];

    neighbours.push([c[0] + 1, c[1], c[2]]);
    neighbours.push([c[0] - 1, c[1], c[2]]);
    neighbours.push([c[0], c[1] + 1, c[2]]);
    neighbours.push([c[0], c[1] - 1, c[2]]);
    neighbours.push([c[0], c[1], c[2] + 1]);
    neighbours.push([c[0], c[1], c[2] - 1]);

    neighbours
}

pub struct BoilingBoulders {
    droplets: HashSet<Coord>,
    max_coord: Coord
}

impl crate::Advent for BoilingBoulders {
    fn new(data: &str) -> Self
        where 
            Self: Sized {                
        
        let droplets: HashSet<Coord> = data.lines().map(|l| {
            let v: Vec<i8> = l.split(',').map(|n| n.parse().unwrap()).collect();
            v.try_into().unwrap()
        }).collect();

        let max_x = droplets.iter().map(|d| d[0]).max().unwrap();
        let max_y = droplets.iter().map(|d| d[1]).max().unwrap();
        let max_z = droplets.iter().map(|d| d[2]).max().unwrap();
        let max_coord: Coord = [max_x, max_y, max_z];
        
        Self { droplets, max_coord }
    }

    fn part_01(&self) -> String {
        let surface_area = self.surface_area(&self.droplets);
        surface_area.to_string()
    }

    fn part_02(&self) -> String {
        let initial_surface_area = self.surface_area(&self.droplets);
        let air_pockets = self.get_air_pockets();

        let mut air_pockets_surface_area = 0;
        for air_pocket in air_pockets {
            let air_pocket_surface_area = self.surface_area(&air_pocket);
            air_pockets_surface_area += air_pocket_surface_area;
        }

        let surface_area = initial_surface_area - air_pockets_surface_area;
        surface_area.to_string()
    }
}

impl BoilingBoulders {
    fn surface_area(&self, spaces: &HashSet<Coord>) -> usize {
        let mut surface_area = 0;
        for coord in spaces.iter() {
            for neighbour_coord in get_neighbour_coords(coord) {
                if !spaces.contains(&neighbour_coord) {
                    surface_area += 1;
                }
            }
        }
        surface_area
    }

    fn empty_neighbours_it<'a> (&'a self, c: &'a Coord) -> impl Iterator<Item = Coord> + 'a {
        get_neighbour_coords(c).into_iter().filter_map(|nc| {
            if self.droplets.contains(&nc) {
                return None;
            }
            Some(nc)
        })
    } 

    fn search_out_of_bounds<R: RangeBounds<i8>>(&self, start: &Coord, bounds: &[R; 3]) -> (bool, HashSet<Coord>) {
        let mut queue: VecDeque<Coord> = VecDeque::new();
        let mut visited: HashSet<Coord> = HashSet::new();

        queue.push_back(start.clone());
        visited.insert(start.clone());

        while let Some(c) = queue.pop_front() {            
            for empty_neighbour in self.empty_neighbours_it(&c) {
                if visited.contains(&empty_neighbour) {
                    continue;
                }

                visited.insert(empty_neighbour.clone());
                for (axis, val) in empty_neighbour.iter().enumerate() {
                    if !bounds[axis].contains(val) {
                        // Is out of bounds / can't be an air bubble
                        return (true, visited);
                    }
                }
                queue.push_back(empty_neighbour.clone());                
            }
        }

        // We have no more empty cells to check
        // This air cell and all visited are an air pocket    
        (false, visited)
    }

    fn get_air_pockets(&self) -> Vec<HashSet<Coord>>{
        let mut air_pockets: Vec<HashSet<Coord>> = vec![];        
        let mut visited: Vec<Coord> = vec![];      

        let bounds = [
            1..self.max_coord[0],
            1..self.max_coord[1],
            1..self.max_coord[2]
        ];

        for x in bounds[0].clone() {
            for y in bounds[1].clone() {
                for z in bounds[2].clone() {
                    let c = [x, y, z];

                    // Is air and has not yet been visited
                    if !self.droplets.contains(&c) && !visited.contains(&c) {
                        let (out_of_bounds, new_visited) = self.search_out_of_bounds(&c, &bounds);
                        if !out_of_bounds {
                            air_pockets.push(new_visited.clone());
                        }
                        visited.extend(new_visited.into_iter());                        
                    }
                }
            }
        }
        air_pockets   
    }
}