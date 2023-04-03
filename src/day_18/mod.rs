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
    droplets: Vec<Coord>
}

impl crate::Advent for BoilingBoulders {
    fn new(data: &str) -> Self
        where 
            Self: Sized {                
        
        let droplets = data.lines().map(|l| {
            let v: Vec<i8> = l.split(',').map(|n| n.parse().unwrap()).collect();
            v.try_into().unwrap()
        }).collect();
        
        Self { droplets }
    }

    fn part_01(&self) -> String {
        let touching_sides_count: usize = self.droplets.iter().map(|c| {
            let neighbour_coords = get_neighbour_coords(c);
            self.droplets.iter().filter(|&oc| {
                oc != c && neighbour_coords.contains(oc)
            }).count()
        }).sum();

        println!("Touching sides: {}", touching_sides_count);

        let surface_area = self.droplets.len() * 6 - touching_sides_count;
        surface_area.to_string()
        // 1.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}