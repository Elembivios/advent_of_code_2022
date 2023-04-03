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
        let touching_sides_count = self.touching_sides_count();
        let surface_area = self.droplets.len() * 6 - touching_sides_count;
        surface_area.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}

impl BoilingBoulders {
    fn touching_sides_count(&self) -> usize {
        self.droplets.iter().map(|c| {
            let neighbour_coords = get_neighbour_coords(c);
            self.droplets.iter().filter(|&oc| {
                oc != c && neighbour_coords.contains(oc)
            }).count()
        }).sum()
    }
}