
pub struct CalorieCounting {
    data: Vec<Vec<u32>>
}

impl crate::Advent for CalorieCounting {
    fn new(data: &str) -> Self {
        let elf_supplies: Vec<&str> = data.lines().collect();
        let data = elf_supplies
            .split(|line| line.is_empty())
            .map(|vals| {
                vals
                    .iter()
                    .map(|val| val.parse().unwrap())
                    .collect()
            })
            .collect();
        CalorieCounting { data }
    }

    fn part_01(&self) -> usize {
        self.data
            .iter()
            .map(|values| {
                values.iter().sum::<u32>()
            }).max().unwrap() as usize
    }
    
    fn part_02(&self) -> usize {
        let mut calories_sum: Vec<u32> = self.data
            .iter()
            .map(|values| {
                values.iter().sum::<u32>()
            }).collect();
        calories_sum.sort();
        calories_sum.iter().rev().take(3).sum::<u32>() as usize
    }
}