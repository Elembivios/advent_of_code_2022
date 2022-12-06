
pub struct TuningTrouble {
    data: Vec<char>
}

impl TuningTrouble {
    // My implementation, that is slower by factor of 10
    // fn same_chars_pos<const SIZE: usize>(&self) -> Option<String> {
    //     let last_chars: Result<[char; SIZE], _> = self.data[0..SIZE].try_into();
    //     match last_chars {
    //         Err(_e) => None,
    //         Ok(mut last_chars) => {
    //             for (i, chr) in self.data.iter().enumerate().skip(SIZE) {
    //                 let num_unique = last_chars.iter().unique().count(); // Count unique
    //                 if num_unique == SIZE {
    //                     return Some(i.to_string());
    //                 }
    //                 last_chars.rotate_left(1);
    //                 last_chars[SIZE - 1] = *chr; // Insert new char in last position
    //             }
    //             None
    //         }
    //     }
    // }

    fn same_chars_pos(&self, size: usize) -> usize {
        self.data
            .windows(size)
            .position(|x| {
                !(1..x.len()).any(|i| x[i..].contains(&x[i-1]))
            })
            .unwrap() + size
    }
}

impl crate::Advent for TuningTrouble {
    fn new(data: &str) -> Self {
        let data: Vec<char> = data.lines().next().unwrap().chars().collect();
        TuningTrouble { data }
    }

    fn part_01(&self) -> String {
        self.same_chars_pos(4).to_string()
    }

    fn part_02(&self) -> String {
        self.same_chars_pos(14).to_string()
    }
}