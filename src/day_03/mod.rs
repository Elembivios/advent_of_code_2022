use std::ops::Index;

pub struct RucksackReorganization {
    data: Vec<(String, String)>
}


fn get_common_char(lhs: &String, rhs: &String) -> Option<char> { 
    for c in lhs.chars() {
        if rhs.contains(c) {
            return Some(c);
        }
    }
    None
}

let lowercase: Vec<char> = ('a'..='z').into_iter().collect();
let uppercase: Vec<char> = ('A'..='Z').into_iter().collect();

fn char_priority(c: char) -> usize {
    let priority = if c.is_ascii_lowercase() {
        LOWERCASE.iter().position(|el| *el == c).unwrap()
    } else {
        UPPERCASE.iter().position(|el| *el == c).unwrap() + 26
    };

    priority
}

impl crate::Advent for RucksackReorganization {
    fn new(data: &str) -> RucksackReorganization {
        let data = data.lines()
            .map(|l| {
                l.split_at(l.len() / 2 )
            }).map(|(lhs, rhs)| {
                (lhs.to_owned(), rhs.to_owned())
            }).collect();
        RucksackReorganization { data }
    }    
    
    fn part_01(&self) -> usize {
        let mut common_chars: Vec<char> = vec![];
        for (lhs, rhs) in &self.data {
            let c = get_common_char(lhs, rhs);
            if let Some(c) = c {
                common_chars.push(c)
            }
        }
        println!("Common chars: {:?}", common_chars);
        1
    }
    
    fn part_02(&self) -> usize {
        2
    }
}