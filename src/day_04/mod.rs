
use std::ops::RangeInclusive;
pub struct CampCleanup {
    data: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>
}

impl crate::Advent for CampCleanup {
    fn new(data: &str) -> Self {
        let data = data
            .lines()
            .map(|l| {
                let pairs: Vec<Vec<u32>> = l.split(",").map(|pair| {
                    pair.split("-").map(|num| num.parse::<u32>().unwrap()).collect()
                }).collect();
                ((pairs[0][0]..=pairs[0][1]), (pairs[1][0]..=pairs[1][1]))
            }).collect();

        // println!("Data: {:?}", data);
        CampCleanup { data }
    }

    fn part_01(&self) -> String {
        let mut fully_contained_sum: usize = 0;
        for pair in &self.data {
            let lhs_ord = pair.0.start().cmp(&pair.1.start());
            let rhs_ord = pair.0.end().cmp(&pair.1.end());
            if lhs_ord != rhs_ord || lhs_ord.is_eq() || rhs_ord.is_eq() {
                fully_contained_sum += 1;
            }
        }
        fully_contained_sum.to_string()
    }

    fn part_02(&self) -> String {
        let mut num_overlap: usize = 0;
        for pair in &self.data {
            let lhs_ord = pair.0.end().cmp(&pair.1.start());
            let rhs_ord = pair.0.start().cmp(&pair.1.end());
            if !(lhs_ord.is_lt() || rhs_ord.is_gt()) {
                num_overlap += 1; 
            }
        }
        num_overlap.to_string()
    }
}