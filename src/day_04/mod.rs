
use core::ops::Range;
pub struct CampCleanup {
    data: Vec<(Range<u32>, Range<u32>)>
}

impl crate::Advent for CampCleanup {
    fn new(data: &str) -> Self {
        let data = data
            .lines()
            .map(|l| {
                let pairs: Vec<Vec<u32>> = l.split(",").map(|pair| {
                    pair.split("-").map(|num| num.parse::<u32>().unwrap()).collect()
                }).collect();
                ((pairs[0][0]..pairs[0][1]), (pairs[1][0]..pairs[1][1]))
            }).collect();

        println!("Data: {:?}", data);
        CampCleanup { data }
    }

    fn part_01(&self) -> usize {
        1
    }

    fn part_02(&self) -> usize {
        2
    }
}