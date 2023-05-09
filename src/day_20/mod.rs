// Upgraded my solution from AxlLind solution from
// https://github.com/AxlLind/AdventOfCode2022/blob/main/src/bin/20.rs


pub struct GrovePositioningSystem {    
    file: Vec<isize>
}

impl crate::Advent for GrovePositioningSystem {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let file = data.lines().map(|l| {
            l.parse().unwrap()
        }).collect();
        Self { file }
    }
    
    fn part_01(&self) -> String {  
        let mut indexes:Vec<usize> = (0..self.file.len()).collect();  
        self.mix_numbers(&self.file, &mut indexes);
        let result = self.find_groove_coordinates(&self.file, &indexes);
        result.to_string()
    }

    fn part_02(&self) -> String {
        let key = 811589153;
        let file: Vec<isize> = self.file.iter().map(|n| *n * key).collect();
        let mut indexes:Vec<usize> = (0..file.len()).collect();
        for _ in 0..10 {
            self.mix_numbers(&file, &mut indexes);
        }
        let result = self.find_groove_coordinates(&file, &indexes);
        result.to_string()
    }
}

impl GrovePositioningSystem {
    fn find_groove_coordinates(&self, file: &Vec<isize>, indexes: &Vec<usize>) -> isize {
        let original_zero_i = file.iter().position(|&i| i == 0).unwrap();
        let zero_i = indexes.iter().position(|&i| i == original_zero_i).unwrap();
        [1000, 2000, 3000].iter().map(|i| {
            file[indexes[(zero_i + i) % indexes.len()]]
        }).sum()
    }

    fn mix_numbers(&self, file: &Vec<isize>, indexes: &mut Vec<usize>) {
        for (i, &x) in file.iter().enumerate() {
            let pos = indexes.iter().position(|&n| n == i).unwrap();
            indexes.remove(pos);
            let new_index = (pos as isize + x).rem_euclid(indexes.len() as isize) as usize;
            indexes.insert(new_index, i);
        }
    }
}