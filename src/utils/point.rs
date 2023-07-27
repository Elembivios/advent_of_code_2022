use std::fmt::{self, Display};
use std::cmp::Ordering;
use std::ops::{Add, Sub, AddAssign, SubAssign};
use anyhow::{Error, anyhow};
use funty::Signed;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord<T> {
    pub x: T,
    pub y: T
}

impl<T> Coord<T> {
    pub fn new(x: T, y: T) -> Self {
        Coord { x, y }
    }

    pub fn get<'a>(&'a self, axis: &Axis) -> &'a T {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y
        }
    }

    pub fn get_mut<'a>(&'a mut self, axis: &Axis) -> &'a mut T {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y
        }
    }
}

impl<T> Coord<T> 
where
    T: Signed
{
    pub fn manhattan_distance(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }    
}

impl<T> Coord<T>
where
    T: num::Integer + Copy
{
    pub fn get_neighbour(&self, direction: &Direction) -> Self {
        match direction {
            Direction::N => Coord::new(self.x, self.y + num::one()),
            Direction::NE => Coord::new(self.x + num::one(), self.y + num::one()),
            Direction::E => Coord::new(self.x + num::one(), self.y),
            Direction::SE => Coord::new(self.x + num::one(), self.y - num::one()),
            Direction::S => Coord::new(self.x, self.y - num::one()),
            Direction::SW => Coord::new(self.x - num::one(), self.y - num::one()),
            Direction::W => Coord::new(self.x - num::one(), self.y),
            Direction::NW => Coord::new(self.x - num::one(), self.y + num::one()),
        }
    }
}

impl<T: Add<Output = T>> Add for Coord<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign> AddAssign for Coord<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y
    }
}

impl<T: SubAssign> SubAssign for Coord<T>
where 
    T: num::Signed
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Coord<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> fmt::Display for Coord<T> 
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> fmt::Debug for Coord<T> 
where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C({:?}, {:?})", self.x, self.y)
    }
}

impl<T: Ord> Ord for Coord<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let y_cmp = self.y.cmp(&other.y);
        match y_cmp {
            Ordering::Equal => {
                self.x.cmp(&other.x)
            },
            _ => y_cmp
        }
    }
}

impl<T: PartialOrd + Ord> PartialOrd for Coord<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> From<&Direction> for Coord<T>
where 
    T: num::Signed
{
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::N  => Coord::new( T::zero(), T::one()), 
            Direction::E  => Coord::new( T::one(),  T::zero()),
            Direction::S  => Coord::new( T::zero(),-T::one()),
            Direction::W  => Coord::new(-T::one(),  T::zero()),
            Direction::NE => Coord::new( T::one(),  T::one()),
            Direction::NW => Coord::new(-T::one(),  T::one()),
            Direction::SE => Coord::new( T::one(), -T::one()),
            Direction::SW => Coord::new(-T::one(), -T::one()),
        }
    }
}

impl<T> From<(T, T)> for Coord<T>
{
    fn from(tuple: (T, T)) -> Self {
        Coord::new( tuple.0, tuple.1 )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction { 
    N, E, S, W,
    NE, NW, SE, SW
}

impl TryFrom<&str> for Direction {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "N" => Ok(Direction::N),
            "NE" => Ok(Direction::NE),
            "E" => Ok(Direction::E),
            "SE" => Ok(Direction::SE),
            "S" => Ok(Direction::S),
            "SW" => Ok(Direction::SW),
            "W" => Ok(Direction::W),
            "NW" => Ok(Direction::NW),
            _ => Err(anyhow!["Invalid value for direction `{}`", value])
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Direction::N => "N",
            Direction::NE => "NE",
            Direction::E => "E",
            Direction::SE => "SE",
            Direction::S => "S",
            Direction::SW => "SW",
            Direction::W => "W",
            Direction::NW => "NW"
        };
        write!(f, "{}", s)
    }
}

pub const TOUCHING_DIRECTIONS: [Direction; 4] = [
    Direction::N, Direction::E, Direction::S, Direction::W
];

pub const DIRECTIONS: [Direction; 8]  = [
    Direction::N, Direction::NE, Direction::E, Direction::SE,
    Direction::S, Direction::SW, Direction::W, Direction::NW
];

impl Direction {
    pub fn affected_axes(&self) -> Vec<Axis> {        
        match self {
            Direction::N | Direction::S => vec![Axis::Y],
            Direction::W | Direction::E => vec![Axis::X],
            _ => vec![Axis::X, Axis::Y]
        }
    }

    pub fn rotate(&self, degrees: isize) -> Self {
        if degrees % 45 != 0 {
            panic!("Invalid value for rotation degrees ({}). Degrees need to be in 45 steps (divisible by 45)", degrees);
        }
        let rotations: isize = degrees / 45;
        let index = DIRECTIONS.iter().position(|d| d == self).unwrap() as isize;
        let new_index = (index + rotations).rem_euclid(DIRECTIONS.len() as isize) as usize;
        DIRECTIONS[new_index]
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Axis { X, Y }

impl Axis {
    pub fn other(&self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X
        }
    }
}


#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Point<T, U> {
    pub coord: Coord<T>,
    pub value: U
}

impl<T, U> Point<T, U> {
    pub fn new(x: T, y: T, value: U) -> Self {
        Point { coord: Coord::new(x, y), value }
    }

    pub fn from_coord(coord: Coord<T>, value: U) -> Self {
        Point { coord, value }
    }
}

impl<T, U> fmt::Display for Point<T, U> 
where
    U: Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone)]
pub struct Grid<V> {
    pub map: Vec<V>,
    pub height: usize,
    pub width: usize
}

impl<V> Grid<V>
{
    pub fn new(map: Vec<Vec<V>>) -> Self {
        let height = map.len();
        let width = map.get(0).unwrap_or(&vec![]).len();
        for (i, row) in map.iter().enumerate() {
            if row.len() != width {
                panic!("Row {} width {} is not the same as all the rest ({})", i, row.len(), width);
            }
        }
        Grid {
            map: map.into_iter().flatten().collect(),
            height, width
        }
    }

    pub fn contains<T>(&self, coord: &Coord<T>) -> bool
    where T: num::Integer + Copy + num::FromPrimitive
    {   
        let h = num::FromPrimitive::from_usize(self.height).unwrap();
        let w = num::FromPrimitive::from_usize(self.width).unwrap();        
        if coord.y >= num::zero() && coord.y < h && coord.x >= num::zero() && coord.x < w {
            true
        } else {
            false
        }
    }

    pub fn get_neighbour(&self, coord: &Coord<usize>, direction: &Direction) -> Option<Coord<usize>> {
        match direction {
            Direction::N => if coord.y == 0 { None } else { Some(Coord::new(coord.x, coord.y - 1)) },            
            Direction::E => if coord.x == self.width - 1 { None } else { Some(Coord::new(coord.x + 1, coord.y)) },
            Direction::S => if coord.y == self.height - 1 { None } else { Some(Coord::new(coord.x, coord.y + 1)) },
            Direction::W => if coord.x == 0 { None } else { Some(Coord::new(coord.x - 1, coord.y)) },
            Direction::NE => if coord.x == self.width - 1 || coord.y == 0 { None } else { Some(Coord::new(coord.x + 1, coord.y - 1))},
            Direction::SE => if coord.x == self.width - 1 || coord.y == self.height - 1 { None } else { Some(Coord::new(coord.x + 1, coord.y + 1))},
            Direction::SW => if coord.x == 0 || coord.y == self.height - 1 { None } else { Some(Coord::new(coord.x - 1, coord.y + 1))},
            Direction::NW => if coord.x == 0 || coord.y == 0 { None } else { Some(Coord::new(coord.x - 1, coord.y - 1))},
        }
    }

    pub fn get_index(&self, coord: &Coord<usize>) -> usize {
        coord.y * self.width + coord.x
    }

    pub fn get_val(&self, coord: &Coord<usize>) -> &V {
        &self.map[self.get_index(coord)]
    }

    pub fn get_val_mut(&mut self, coord: &Coord<usize>) -> &mut V {
        &mut self.map[coord.y * self.width + coord.x]
    }

    pub fn get_point(&self, coord: &Coord<usize>) -> Point<usize, &V> {
        Point::from_coord(coord.clone(), self.get_val(coord))
    }

    pub fn get_point_mut(&mut self, coord: &Coord<usize>) -> Point<usize, &mut V> {
        Point::from_coord(coord.clone(), self.get_val_mut(coord))
    }

    pub fn neighbour_coords(&self, coord: &Coord<usize>) -> Vec<Coord<usize>> {
        TOUCHING_DIRECTIONS
            .iter()
            .filter_map(|direction| {
                self.get_neighbour(coord, direction)
            }).collect()
    }

    pub fn neigbour_coords_optional(&self, coord: &Coord<usize>) -> Vec<Option<Coord<usize>>> {
        TOUCHING_DIRECTIONS
            .iter()
            .map(|direction| {
                self.get_neighbour(coord, direction)
            }).collect()
    }

    /// Get's neighbour coords of a specified coordinate. If the neighbour
    /// coordinate is off the edge of map, it returns the one on the opposite 
    /// end of map.
    pub fn neighbour_coords_wrapping(&self, coord: &Coord<usize>) -> Vec<Coord<usize>> {
        let mut neighbours = vec![];
        // North
        if coord.y == 0 {
            neighbours.push(Coord::new(coord.x, self.height - 1));
        } else {
            neighbours.push(Coord::new(coord.x, coord.y - 1));
        }
        // East
        if coord.x == self.width - 1 {
            neighbours.push(Coord::new(0, coord.y));
        } else {
            neighbours.push(Coord::new(coord.x + 1, coord.y));
        }
        // South
        if coord.y == self.height - 1 {
            neighbours.push(Coord::new(coord.x, 0));
        } else {
            neighbours.push(Coord::new(coord.x, coord.y + 1));
        }

        // West
        if coord.x == 0 {
            neighbours.push(Coord::new(self.width - 1, coord.y));
        } else {
            neighbours.push(Coord::new(coord.x - 1, coord.y));
        }        
        neighbours
    }

    pub fn iter_values(&self) -> impl Iterator<Item=&V> {
        self.map.iter()
    }

    pub fn iter_coords(&self) -> impl Iterator<Item=Coord<usize>> + '_ {
        (0..self.height).into_iter().map(move |y| (0..self.width).into_iter().map(move |x| Coord::new(x, y))).flatten()
    }

    // pub fn iter_coords_mut(&mut self) -> impl Iterator<Item=
    

    pub fn iter_points(&self) -> impl Iterator<Item=Point<usize, &V>> + '_ {
        self.iter_coords().map(|c| {
            let val = self.get_val(&c);
            Point::from_coord(c, val)
        })
    }
    
    pub fn iter_points_mut(&mut self) -> impl Iterator<Item=Point<usize, &mut V>> {
        self.map
            .iter_mut()
            .enumerate()
            .map(|(i, v)| {
                let y = i / self.width;
                let x = i % self.width;
                let c = Coord::new(x, y);
                Point::from_coord(c, v)
            })
    }

    pub fn direction_iter(&self, direction: Direction, current_coord: Coord<usize>) -> GridDirectionIterator {
        let axis = direction.affected_axes()[0];
        GridDirectionIterator {
            height: self.height,
            width: self.width,
            direction, current_coord, axis
        }
    }

    pub fn wrapped_direction_iter(&self, direction: Direction, current_coord: Coord<usize>) -> GridWrappedDirectionIterator {
        let axis = direction.affected_axes()[0];
        GridWrappedDirectionIterator { 
            height: self.height, 
            width: self.width, 
            direction, current_coord, axis 
        }
    }
}

impl<V> Grid<V>
where V: Copy
 {
    pub fn rotate(&mut self, clockwise: bool) {
        let new_map: Vec<V> = if clockwise {
            (0..self.width).map(|x| {
                (0..self.height).rev().map(|y| {
                    self.map[y * self.width + x]
                }).collect::<Vec<V>>()
            }).flatten().collect()
        } else {
            (0..self.width).rev().map(|x| {
                (0..self.height).map(|y| {
                    self.map[y * self.width + x]
                }).collect::<Vec<V>>()
            }).flatten().collect()
        };
        self.map = new_map;
        let width = self.width;
        self.width = self.height;
        self.height = width;        
    }
}


impl<V> Grid<V> 
where 
    V: Display
{
    pub fn display_with_points(&self, points: Vec<Coord<usize>>, display_char: char) {
        print!("\n");
        for y in 0..self.height {
            for x in 0..self.width {
                let current = Coord::new(x, y);
                if points.contains(&current) {
                    print!("{}", display_char);
                } else {
                    let p = self.get_point(&current);
                    print!("{}", p);
                }
            }
            print!("\n");
        }
    }
}

impl<V> fmt::Display for Grid<V> 
where
    V: Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for y in 0..self.height {            
            for x in 0..self.width {
                let v = self.get_val(&Coord::new(x, y));
                    write!(f, "{}", v)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}


// Endles iterator of the grid in specified direction
// When it gets to the edge it jumps to the other side and 
// continues iterating in that direction.


pub struct GridDirectionIterator {
    height: usize,
    width: usize,
    direction: Direction,
    current_coord: Coord<usize>,
    axis: Axis,
}

impl Iterator for GridDirectionIterator {
    type Item = Coord<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.current_coord.get_mut(&self.axis);
        match self.direction {
            Direction::S => {
                if *val == self.height - 1 { return None; } else { *val += 1; }
            }
            Direction::E => {
                if *val == self.width - 1 { return None; } else { *val += 1; }
            },
            Direction::N => {
                if *val == 0 { return None; } else { *val -= 1; }
            }
            Direction::W => {
                if *val == 0 { return None; } else { *val -= 1; }
            },
            _ => unimplemented!("Iterator for direction {:?} is not implemented.", self.direction)
        }        
        Some(self.current_coord.clone())
    }
}

pub struct GridWrappedDirectionIterator {
    height: usize,
    width: usize,
    direction: Direction,
    current_coord: Coord<usize>,
    axis: Axis,
}

impl Iterator for GridWrappedDirectionIterator {
    type Item = Coord<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.current_coord.get_mut(&self.axis);
        match self.direction {
            Direction::S => {
                if *val == self.height - 1 { *val = 0; } else { *val += 1; }
            }
            Direction::E => {
                if *val == self.width - 1 { *val = 0; } else { *val += 1; }
            },
            Direction::N => {
                if *val == 0 { *val = self.height - 1; } else { *val -= 1; }
            }
            Direction::W => {
                if *val == 0 { *val = self.width - 1; } else { *val -= 1; }
            },
            _ => unimplemented!("Iterator for direction {:?} is not implemented.", self.direction)
        }        
        Some(self.current_coord.clone())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn construct_grid() -> Grid<char> {
        let map = (0..10).into_iter().map(|_y| {
            (0..10).into_iter().map(|_x| {
                '.'
            }).collect()
        }).collect();
        Grid::new(map)
    }

    #[test]
    fn test_direction_rotation() {
        use Direction::*;

        assert_eq!(N.rotate(90), E);
        assert_eq!(N.rotate(-90), W);
        assert_eq!(N.rotate(180), S);
        assert_eq!(N.rotate(-180), S);
        assert_eq!(N.rotate(-45), NW);
        assert_eq!(N.rotate(45), NE);
        assert_eq!(N.rotate(135), SE);
        assert_eq!(N.rotate(-135), SW);
        assert_eq!(N.rotate(360), N);

        assert_eq!(S.rotate(90), W);
        assert_eq!(S.rotate(-90), E);

    }

    #[test]
    fn test_coord_neighbours() {
        let c = Coord::new(0, 0);
        let neighbours: Vec<_> = DIRECTIONS.iter().map(|dir| {
            c.get_neighbour(dir)
        }).collect();  
        assert_eq!(neighbours, vec![
            Coord::new(0, 1),
            Coord::new(1, 1),
            Coord::new(1, 0),
            Coord::new(1, -1),
            Coord::new(0, -1),
            Coord::new(-1, -1),
            Coord::new(-1, 0),
            Coord::new(-1, 1),
        ])
    }

    #[test]
    fn test_contains() {
        let grid = construct_grid();
        println!("{}", grid);
        let c = Coord::new(0, 0);
        assert_eq!(grid.contains(&c), true);
        let c = Coord::new(-1, -1);
        assert_eq!(grid.contains(&c), false);
        let c: Coord<usize> = Coord::new(1, 1);
        assert_eq!(grid.contains(&c), true);
    }
}