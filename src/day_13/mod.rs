use std::cmp::Ordering;
use std::str::FromStr;
use anyhow::{Error, Result};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    Number(u8),
    List(Vec<Packet>)
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Number(lhs), Number(rhs)) => lhs.cmp(rhs),
            (List(lhs), List(rhs)) => lhs.cmp(rhs),
            (Number(lhs), List(rhs)) => [Number(*lhs)][..].cmp(rhs),
            (List(lhs), Number(rhs)) => lhs.as_slice().cmp(&[Number(*rhs)])
        }
    }
}
impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::List(list) => {
                write!(f, "[")?;
                
                for (i, p) in list.iter().enumerate() {
                    write!(f, "{p}")?;
                    if i != list.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("10", "a");        
        let mut packet = Self::List(vec![]);
        let mut chars = s.chars();

        while let Some(c) = chars.next() {
            match c {
                '[' => {
                    let mut s = String::new();
                    let mut depth = 1;

                    while depth > 0 {
                        let c = chars.next().unwrap();
                        match c {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => {}
                        }
                        s.push(c);
                    }

                    if let Ok(sub_packet)  = s[..s.len() - 1].parse() {
                        if let Self::List(list) = &mut packet {
                            list.push(sub_packet);
                        }
                    }
                },
                ',' => {},
                'a' => {
                    if let Self::List(list) = &mut packet {
                        list.push(Self::Number(10));
                    }
                },
                _ => {
                    if let Self::List(list) = &mut packet {
                        list.push(Self::Number(c.to_digit(10).unwrap() as u8));
                    }
                }
            }
        }        
        Ok(packet)
    }
}


pub struct DistressSignal {
    packets: Vec<Vec<Packet>>
}

impl crate::Advent for DistressSignal {
    fn new(data: &str) -> Self {
        let lines: Vec<&str> = data.lines().collect();
        let packets: Vec<_> = lines
            .split(|l| l.is_empty())
            .map(|pair| {                
                pair.iter().map(|part| {
                    let packet = part.parse::<Packet>().unwrap();
                    packet
                }).collect::<Vec<Packet>>()
            }).collect();     
        DistressSignal { 
            packets
        }
    }

    fn part_01(&self) -> String {
        let mut inorder_packets: Vec<usize> = vec![];
        for (i, pair) in self.packets.iter().enumerate() {
            let left = &pair[0];
            let right = &pair[1];
            if left < right {
                inorder_packets.push(i + 1);
            }
        }
        inorder_packets.iter().sum::<usize>().to_string()
    }

    fn part_02(&self) -> String {
        let div1: Packet = "[[2]]".parse().unwrap();
        let div2: Packet = "[[6]]".parse().unwrap();
        let mut packets: Vec<Packet> = self.packets.iter().flat_map(|p| p.clone()).collect();
        packets.push(div1.clone());
        packets.push(div2.clone());
        packets.sort_unstable();

        let pos1 = packets.binary_search(&div1).unwrap() + 1;
        let pos2 = packets.binary_search(&div2).unwrap() + 1;

        (pos1 * pos2).to_string()
    }
}
