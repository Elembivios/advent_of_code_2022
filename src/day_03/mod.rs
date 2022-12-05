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

fn get_common_chars(items: &Vec<String>) -> Option<char> { 
    let mut remaining_chars: Vec<char> = items.first().unwrap().chars().collect();
    remaining_chars.sort_unstable();
    remaining_chars.dedup();
    

    for item in &items[1..] {
        let mut new_chars: Vec<char> = vec![];

        for c in &remaining_chars {
            if item.chars().any(|el| *c == el) {
                new_chars.push(*c);
            }
        }

        remaining_chars.clear();
        remaining_chars = new_chars.clone();
    }

    if remaining_chars.len() == 1 {
        return Some(remaining_chars[0]);
    }

    None
}

fn char_priority(c: char) -> usize {
    let priority = if c.is_ascii_lowercase() {
        ('a'..='z').into_iter().position(|el| el == c).unwrap() + 1
    } else {
        ('A'..='Z').into_iter().position(|el| el == c).unwrap() + 27
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
    
    fn part_01(&self) -> String {
        let mut common_chars: Vec<char> = vec![];
        for (lhs, rhs) in &self.data {
            let c = get_common_char(lhs, rhs);
            if let Some(c) = c {
                common_chars.push(c)
            }
        }
        common_chars.iter()
            .map(|c| char_priority(*c))
            .sum::<usize>().to_string()
    }
    
    fn part_02(&self) -> String {
        let mut common_chars: Vec<char> = vec![];
        for items in self.data.chunks(3) {
            let lines: Vec<String> = items
                .iter()
                .map(|(lhs, rhs)| {
                    let mut new_str = String::from(lhs);
                    new_str.push_str(rhs);
                    new_str
                })
                .collect();
            let common_char = get_common_chars(&lines);
            common_chars.push(common_char.unwrap())            
        }

        common_chars.iter()
            .map(|c| char_priority(*c))
            .sum::<usize>().to_string()     
    }
}