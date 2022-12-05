pub struct SupplyStacks {
    stacks: Vec<Vec<char>>,
    instructions: Vec<(usize, usize, usize)>
}

impl crate::Advent for SupplyStacks {
    fn new(data: &str) -> Self {
        let data: Vec<&str> = data.lines().collect();
        let data: Vec<_> = data.split(|e| e.is_empty()).collect();
        
        let mut stacks: Vec<Vec<char>> = vec![];
        let length = data[0].iter().rev().next().unwrap().len();
        let width = (length + 1) / 4;        
        for _ in 0..width {
            stacks.push(vec![]);
        }

        data[0].iter().rev().skip(1).for_each(|line| {  
            let all_chars: Vec<char> = line.chars().collect();
            all_chars
                .chunks(4)
                .enumerate()
                .filter(|(_i, chrs)| chrs[1] != ' ')                
                .for_each(|(i, chrs)| {
                    stacks[i].push(chrs[1])
                });
        });

        let instructions: Vec<(usize, usize, usize)> = data[1].iter().map(|l| {
            let parts: Vec<_> = l.split(' ').collect();
            (
                parts[1].parse().unwrap(), 
                parts[3].parse::<usize>().unwrap() - 1, 
                parts[5].parse::<usize>().unwrap() - 1
            )
        }).collect();

        SupplyStacks {
            stacks,
            instructions
        }
    }
    fn part_01(&self) -> String {
        let mut stacks = self.stacks.clone();
        for (quant, from, to) in &self.instructions {
            let quant = 0.max(stacks[*from].len() as i32 - *quant as i32) as usize;            
            let mut drained: Vec<char> = stacks[*from].drain(quant..).rev().collect();
            stacks[*to].append(&mut drained);
        }
        let mut result: Vec<char> = vec![];
        for stack in stacks {
            result.push(stack.last().unwrap_or(&' ').clone())
        }
        result.into_iter().collect()
    }

    fn part_02(&self) -> String {
        let mut stacks = self.stacks.clone();
        for (quant, from, to) in &self.instructions {
            let quant = 0.max(stacks[*from].len() as i32 - *quant as i32) as usize;            
            let mut drained: Vec<char> = stacks[*from].drain(quant..).collect();
            stacks[*to].append(&mut drained);
        }
        let mut result: Vec<char> = vec![];
        for stack in stacks {
            result.push(stack.last().unwrap_or(&' ').clone())
        }
        result.into_iter().collect()
    }
}