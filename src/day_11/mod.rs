use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

pub struct MonkeyInTheMiddle {
    monkeys: Vec<Monkey>
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u128>,
    operation: Rc<dyn Fn(u128) -> u128>,
    test_num: u128,
    true_index: usize,
    false_index: usize,
}

impl Monkey {
    fn test(&self, worry_level: u128) -> usize {
        if worry_level % self.test_num == 0 {
            self.true_index
        } else {
            self.false_index
        }        
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("test_num", &self.test_num)
            .field("true_index", &self.true_index)
            .field("false_index", &self.false_index)
            .finish()
    }
}

impl crate::Advent for MonkeyInTheMiddle {
    fn new(data: &str) -> Self {
        let lines: Vec<&str> = data.lines().collect();
        let monkeys = lines
            .split(|line| line.is_empty())
            .map(|monkey_data| {
                let mut it = monkey_data.iter().skip(1);
                let (_lhs, rhs) = it.next().unwrap().split_once(": ").unwrap();
                let items: VecDeque<u128> = rhs.split(", ").map(|n| n.parse().unwrap()).collect();
                let (_lhs, rhs) = it.next().unwrap().split_once("= ").unwrap();                
                let operation: Vec<_> = rhs.split(" ").collect();
                let func: Rc<dyn Fn(u128) -> u128> = match operation[2] {
                    "old" => {
                        match operation[1] {
                            "*" => Rc::new(move |old| old * old),
                            "+" => Rc::new(move |old| old + old),
                            _ => panic!("Invalid sign!")
                        }
                    },
                    num => {
                        let num: u128 = num.parse().unwrap();
                        match operation[1] {
                            "*" => Rc::new(move |old| old * num),
                            "+" => Rc::new(move |old| old + num),
                            _ => panic!("Invalid sign!")
                        }
                    }
                };
                let test_num: u128 = it.next().unwrap().split(" ").last().unwrap().parse().unwrap();
                let true_index: usize = it.next().unwrap().split(" ").last().unwrap().parse().unwrap();
                let false_index: usize = it.next().unwrap().split(" ").last().unwrap().parse().unwrap();

                Monkey {
                    items,
                    operation: func,
                    test_num,
                    true_index,
                    false_index
                }
            }).collect();
        MonkeyInTheMiddle { monkeys }
    }

    fn part_01(&self) -> String {
        let monkeys: Vec<_> = self.monkeys.clone().into_iter().map(|m| RefCell::new(m)).collect();
        let mut inspected: Vec<usize> = (0..monkeys.len()).map(|_| 0).collect();

        for _step in 0..20 {
            
            for (i, monkey) in monkeys.iter().enumerate() {
                let mut monka = monkey.borrow_mut();
                while let Some(item) = monka.items.pop_front() {
                    let mut worry_level = (monka.operation)(item);
                    inspected[i] += 1;
                    worry_level /= 3;                                        
                    let monkey_index = monka.test(worry_level);                    
                    monkeys[monkey_index].borrow_mut().items.push_back(worry_level);
                }
            }        
        }
        inspected.sort_unstable();
        inspected.iter().rev().take(2).fold(1, |axx, x| axx * x).to_string()
    }

    fn part_02(&self) -> String {
        let max_div = self.monkeys.iter().map(|m| m.test_num).fold(1, |agg, x| agg * x);

        let monkeys: Vec<_> = self.monkeys.clone().into_iter().map(|m| RefCell::new(m)).collect();
        let mut inspected: Vec<usize> = (0..monkeys.len()).map(|_| 0).collect();
        for _step in 0..10_000 {            
            for (i, monkey) in monkeys.iter().enumerate() {
                let mut monka = monkey.borrow_mut();
                while let Some(item) = monka.items.pop_front() {
                    let mut worry_level = (monka.operation)(item);
                    inspected[i] += 1;
                    if worry_level > max_div {
                        worry_level %= max_div;
                    }
                    let monkey_index = monka.test(worry_level);                    
                    monkeys[monkey_index].borrow_mut().items.push_back(worry_level);
                }
            }
        }
        inspected.sort_unstable();
        inspected.iter().rev().take(2).fold(1, |axx, x| axx * x).to_string()
    }
}
