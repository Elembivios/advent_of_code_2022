use std::str::FromStr;
use anyhow::{Error, Result, Context};

pub struct CathodeRayTube {
    instructions: Vec<Instruction>
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add(isize),
    Noop
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Self::Add(_) => 2,
            Self::Noop => 1
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Self::Noop),
            _ => {
                let string = s.split_once(" ").context("Invalid command")?;
                let num: isize = string.1.parse()?;
                Ok(Self::Add(num))
            }
        }
    }
}

impl crate::Advent for CathodeRayTube {
    fn new(data: &str) -> Self {
        let instructions = data
            .lines()
            .map(|l| {
                Instruction::from_str(l).unwrap()
            }).collect();
        CathodeRayTube { instructions }
    }

    fn part_01(&self) -> String {
        let mut x: isize = 1;
        let mut cycles: usize = 0;
        let mut results: Vec<isize> = vec![];
        for instruction in self.instructions.iter() {
            let next_instruction_cycles = cycles + instruction.cycles();
            for i in cycles + 1..=next_instruction_cycles {
                if (i - 20) % 40 == 0 {
                    results.push(i as isize * x);
                }
            }
            
            match instruction {
                Instruction::Add(val) => {
                    x += val;
                },
                _ => {}
            }
            cycles = next_instruction_cycles;
        }
        results.iter().sum::<isize>().to_string()
    }

    fn part_02(&self) -> String {
        let mut x: isize = 1;
        let mut cycles: usize = 0;
        let mut crt: [char; 40 * 6] = ['.'; 40 * 6];
        for instruction in self.instructions.iter() {
            let next_instruction_cycles = cycles + instruction.cycles();
            for i in cycles..next_instruction_cycles {
                let vertical_pos = i % 40;
                if (x - 1..=x + 1).contains(&(vertical_pos as isize)) {
                    crt[i] = '#';
                } else {
                    crt[i] = '.';
                }    
            }
            
            match instruction {
                Instruction::Add(val) => {
                    x += val;
                },
                _ => {}
            }
            cycles = next_instruction_cycles;
        }
        let mut result: String = String::from("\n");
        for y in 0..6 {
            let row: String = (0..40).map(|x| {
                let index = y * 40 + x;
                crt[index]
            }).collect();
            result.push_str(&row);
            result.push_str("\n");
        }        
        result
    }
}
