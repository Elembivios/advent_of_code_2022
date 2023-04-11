// 3284 -- too high

use std::{collections::VecDeque, ops::RangeBounds};

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
    droplets: Vec<Coord>,
    max_coord: Coord
}

impl crate::Advent for BoilingBoulders {
    fn new(data: &str) -> Self
        where 
            Self: Sized {                
        
        let droplets: Vec<Coord> = data.lines().map(|l| {
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
        let touching_sides_count = self.touching_sides_count(&self.droplets);
        let surface_area = self.droplets.len() * 6 - touching_sides_count;
        surface_area.to_string()
    }

    fn part_02(&self) -> String {
        let touching_sides_count = self.touching_sides_count(&self.droplets);
        let surface_area = self.droplets.len() * 6 - touching_sides_count;

        let air_pockets = self.get_air_pockets();

        let mut air_pockets_surface = 0;
        for air_pocket in air_pockets {
            let ap_touching_sides_count = self.touching_sides_count(&air_pocket);
            let ap_surface = air_pocket.len() * 6 - ap_touching_sides_count;
            air_pockets_surface += ap_surface;
        }

        let result = surface_area - air_pockets_surface;
        result.to_string()
    }
}

impl BoilingBoulders {
    fn touching_sides_count(&self, spaces: &Vec<Coord>) -> usize {
        spaces.iter().map(|c| {
            let neighbour_coords = get_neighbour_coords(c);
            spaces.iter().filter(|&oc| {
                oc != c && neighbour_coords.contains(oc)
            }).count()
        }).sum()
    }   

    fn empty_neighbours_it<'a> (&'a self, c: &'a Coord) -> impl Iterator<Item = Coord> + 'a {
        get_neighbour_coords(c).into_iter().filter_map(|nc| {
            match self.droplets.iter().position(|&d| d == nc) {
                Some(_pos) => None,
                None => Some(nc)
            }
        })
    } 

    fn search_out_of_bounds<R: RangeBounds<i8>>(&self, start: &Coord, bounds: &[R; 3]) -> (bool, Vec<Coord>) {
        let mut queue: VecDeque<Coord> = VecDeque::new();
        let mut visited: Vec<Coord> = vec![];

        queue.push_back(start.clone());
        visited.push(start.clone());

        while let Some(c) = queue.pop_front() {            
            for empty_neighbour in self.empty_neighbours_it(&c) {
                if visited.contains(&empty_neighbour) {
                    continue;
                }

                visited.push(empty_neighbour.clone());
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

    fn get_air_pockets(&self) -> Vec<Vec<Coord>>{
        let mut air_pockets: Vec<Vec<Coord>> = vec![];        
        let mut visited: Vec<Coord> = vec![];      

        let bounds = [
            0..=self.max_coord[0],
            0..=self.max_coord[1],
            0..=self.max_coord[2]
        ];

        for x in 1..self.max_coord[0] {
            for y in 1..self.max_coord[1] {
                for z in 1..self.max_coord[2] {
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