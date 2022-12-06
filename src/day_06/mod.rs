// use anyhow::{Result, bail};

pub struct TuningTrouble {
    data: Vec<char>
}

impl TuningTrouble {
    // My implementation, that is slower by factor of 10 :,(
    // fn same_chars_pos_02<const SIZE: usize>(&self) -> Result<String> {
    //     let mut last_chars: [char; SIZE] = self.data[0..SIZE].try_into()?;
    //     for (i, chr) in self.data.iter().enumerate().skip(SIZE) {
    //         let num_unique = last_chars.iter().count(); // Count unique
    //         if num_unique == SIZE {
    //             return Ok(i.to_string());
    //         }
    //         last_chars.rotate_left(1);
    //         last_chars[SIZE - 1] = *chr; // Insert new char in last position
    //     }
    //     bail!("Could not find {} of the same chars.", SIZE);
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