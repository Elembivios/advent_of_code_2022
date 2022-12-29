use std::fmt;
use std::cmp::Ordering;
use std::ops::{Add, Sub, AddAssign, SubAssign};

#[derive(Clone, PartialEq, Eq, Hash)]
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

pub enum Direction { 
    N, E, S, W,
    NE, NW, SE, SW
}

impl Direction {
    pub fn affected_axes(&self) -> Vec<Axis> {        
        match self {
            Direction::N | Direction::S => vec![Axis::Y],
            Direction::W | Direction::E => vec![Axis::X],
            _ => vec![Axis::X, Axis::Y]
        }
    }
}

pub enum Axis { X, Y }

impl Axis {
    pub fn other(&self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X
        }
    }
}


#[derive(PartialEq, Eq, Debug, Hash)]
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

pub struct Grid<V> {
    pub map: Vec<Point<usize, V>>,
    pub height: usize,
    pub width: usize
}

impl<V> Grid<V>
{
    pub fn new(map: Vec<Vec<Point<usize, V>>>) -> Self {
        let height = map.len();
        let width = map.get(0).unwrap_or(&vec![]).len();

        Grid {
            map: map.into_iter().flatten().collect(),
            height, width
        }
    }

    pub fn get_point(&self, coord: &Coord<usize>) -> &Point<usize, V> {
        &self.map[coord.y * self.width + coord.x]
    }

    pub fn get_neighbours(&self, coord: &Coord<usize>) -> Vec<&Point<usize, V>> {
        let mut neighbours = vec![];
        if coord.x > 0 {
            neighbours.push(self.get_point(
                &Coord::new(coord.x - 1, coord.y)
            ));
        }

        if coord.x < self.width - 1 {
            neighbours.push(self.get_point(
                &Coord::new(coord.x + 1, coord.y)
            ));
        }

        if coord.y > 0 {
            neighbours.push(self.get_point(
                &Coord::new(coord.x, coord.y - 1)
            ));
        }

        if coord.y < self.height - 1 {
            neighbours.push(self.get_point(
                &Coord::new(coord.x, coord.y + 1)
            ));
        }
        neighbours
    }
}