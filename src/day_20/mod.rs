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
        let file: Vec<isize> = self.file.clone();   
        let indexes:Vec<usize> = file.iter().enumerate().map(|(i, _v)| i).collect();     
        let indexes = self.mix_numbers(&file, indexes);
        let new_values = self.generate_new_from_indexes(&file, &indexes);
        let result = self.find_groove_coordinates(new_values);
        result.to_string()
    }

    fn part_02(&self) -> String {
        let key = 811589153;
        let file: Vec<isize> = self.file.iter().map(|n| *n * key).collect();
        let mut indexes:Vec<usize> = file.iter().enumerate().map(|(i, _v)| i).collect();
        for _ in 0..10 {
            indexes = self.mix_numbers(&file, indexes);
        }        
        let new_values = self.generate_new_from_indexes(&file, &indexes);
        let result = self.find_groove_coordinates(new_values);
        result.to_string()
    }
}

impl GrovePositioningSystem {
    fn generate_new_from_indexes(&self, original_file: &Vec<isize>, new_indexes: &Vec<usize>) -> Vec<isize> {
        let mut new_values: Vec<isize> = Vec::with_capacity(original_file.len());
        let mut new_indexes: Vec<(usize, &usize)> = new_indexes.iter().enumerate().collect();
        new_indexes.sort_by(|a, b| a.1.cmp(b.1));

        for (original, _new) in new_indexes {
            new_values.push(original_file[original]);
        }
        new_values
    }

    fn find_groove_coordinates(&self, values: Vec<isize>) -> isize {
        let look_at_after: Vec<usize> = vec![1000, 2000, 3000];
        let zero_index = values.iter().position(|x| *x == 0).unwrap();
        let mut result = 0;
        for look_at in look_at_after {
            let capped = (zero_index + look_at) % (values.len());
            result += values[capped];
        }
        result
    }
    // 4265712588168
    fn mix_numbers(&self, file: &Vec<isize>, mut indexes: Vec<usize>) -> Vec<usize> {
        let original_file = file.clone();
        
        for (i, x) in original_file.iter().enumerate() {
            if x.is_positive() {                
                let current_index = indexes[i];  
                let temp_index = current_index + (*x as usize % (original_file.len() - 1));
                let new_index = if temp_index > original_file.len() - 1 {
                    temp_index % (original_file.len() - 1)
                } else {
                    temp_index
                };
                if new_index == current_index {
                    continue;
                }
                let (min, max, add) = if current_index < new_index {
                    (current_index, new_index, -1)
                } else {
                    (new_index, current_index, 1)
                };
                indexes.iter_mut().filter(|ni| {
                    (min..=max).contains(*ni)
                }).for_each(|ni| {
                    *ni = (*ni as isize + add) as usize;
                });
                indexes[i] = new_index;   
            } else if x.is_negative() {
                let current_index = indexes[i];                
                let temp_index = (current_index as isize + *x) % (original_file.len() as isize - 1);
                let new_index = if temp_index.is_negative() || temp_index == 0 {
                    (original_file.len() as isize - 1 + temp_index) as usize                    
                } else {
                    temp_index as usize
                };
                if new_index == current_index {
                    continue;
                }
                let (min, max, add) = if current_index < new_index {
                    (current_index, new_index, -1)
                } else {
                    (new_index, current_index, 1)
                };
                indexes.iter_mut().filter(|ni| {
                    (min..=max).contains(*ni)
                }).for_each(|ni| {
                    *ni = (*ni as isize + add) as usize;
                });
                indexes[i] = new_index;   
            }        
        }
        indexes
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn negative_module() {
        assert_eq!(-2, -20 % 18);
        assert_eq!(15, 18 - 1 - 2);

        let x: isize = -3;
        let i: isize = 1;
        assert_eq!(i + x, -2);
    }
}