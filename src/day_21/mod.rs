use std::collections::HashMap;


pub struct MonkeyMath {
    monkeys: HashMap<String, MonkeySay>
}

// 1705 -- too low
#[derive(Clone, Debug)]
enum MonkeySay {
    Value(isize),
    Expression((String, char, String))
}

fn add(a: isize, b: isize) -> isize { a + b }
fn sub(a: isize, b: isize) -> isize { a - b }
fn div(a: isize, b: isize) -> isize { a / b }
fn mul(a: isize, b: isize) -> isize { a * b }

fn get_operation(operator: char) -> impl Fn(isize, isize) -> isize {
    match operator {
        '+' => add,
        '-' => sub,
        '/' => div,
        '*' => mul,
        _ => panic!("Invalid operator")
    }
}

#[derive(Debug)]
enum Equation {
    Name(String),
    Value(isize),
    Expression((Box<Equation>, char, Box<Equation>))
}

impl crate::Advent for MonkeyMath {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let monkeys = data.lines().map(|l| {
            let (monkey_name, expression_str) = l.split_once(": ").unwrap();

            let expression: Result<isize, _> = expression_str.parse();
            let expression = match expression {
                Ok(val) => MonkeySay::Value(val),
                Err(_) => {
                    let mut it = expression_str.split(" ");
                    let name1 = it.next().unwrap().to_owned();
                    let operator = it.next().unwrap().chars().next().unwrap();
                    let name2 = it.next().unwrap().to_owned();
                    MonkeySay::Expression((name1, operator, name2))
                }
            };
            (monkey_name.to_owned(), expression)
        }).collect();
        Self { monkeys }
    }

    fn part_01(&self) -> String {
        let result = evaluate_expression(&self.monkeys, "root");        
        result.to_string()
    }

    fn part_02(&self) -> String {
        let mut humn_path : Vec<Equation> = vec![];
        get_equation_for_humn(&self.monkeys, "root", &mut humn_path);
        
        let final_step = humn_path.pop().unwrap();
        let target = match final_step {
            Equation::Expression((op1, _, op2)) => {
                match (*op1, *op2) {
                    (Equation::Value(val), _)  => val,
                    (_, Equation::Value(val)) => val,
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        };

        let result = solve_equation(target, humn_path);
        result.to_string()
    }
}

fn evaluate_expression(monkeys: &HashMap<String, MonkeySay>, monkey_name: &str) -> isize {
    match &monkeys[monkey_name] {
        MonkeySay::Value(val) => *val,
        MonkeySay::Expression((name1, operator, name2)) => {
            let operation = get_operation(*operator);

            operation(
                evaluate_expression(monkeys, name1),
                evaluate_expression(monkeys, name2)
            )
        }
    }
}

fn get_equation_for_humn(monkeys: &HashMap<String, MonkeySay>, monkey_name: &str, humn_equation: &mut Vec<Equation>) -> Option<isize> {
    if monkey_name == "humn" {
        return None;
    }
    match &monkeys[monkey_name] {
        MonkeySay::Value(val) => Some(*val),
        MonkeySay::Expression((name1, operator, name2)) => {
            let lhs = get_equation_for_humn(monkeys, name1, humn_equation);
            let rhs = get_equation_for_humn(monkeys, name2, humn_equation);
            let operation = get_operation(*operator);
            match (lhs, rhs) {
                (Some(lhs), Some(rhs)) => Some(operation(lhs, rhs)),
                (None, Some(rhs)) => {
                    humn_equation.push(
                        Equation::Expression((
                            Box::new(Equation::Name(name1.clone())),
                            *operator,
                            Box::new(Equation::Value(rhs))
                        ))
                    );
                    None
                },
                (Some(lhs), None) => {
                    humn_equation.push(
                        Equation::Expression((
                            Box::new(Equation::Value(lhs)),
                            *operator,                            
                            Box::new(Equation::Name(name2.clone())),
                        ))
                    );
                    None
                },
                _ => unreachable!()
            }
        }
    }
}

fn solve_equation(target: isize, humn_path: Vec<Equation>) -> isize {
    let mut result = target;
    for step in humn_path.into_iter().rev() {
        match step {
            Equation::Expression((lhs, operator, rhs)) => {
                result = match (*lhs, *rhs) {
                    (Equation::Value(val), _) => {
                        match operator {
                            '+' => sub(result, val),
                            '-' => sub(val, result),
                            '*' => div(result, val),
                            '/' => div(val, result),
                            _ => unreachable!()
                        }                            
                    },
                    (_, Equation::Value(val)) => {
                        match operator {
                            '+' => sub(result, val),
                            '-' => add(result, val),
                            '*' => div(result, val),
                            '/' => mul(result, val),
                            _ => unreachable!()

                        }
                    },
                    _ => unreachable!()
                };
            },
            _ => unreachable!()
        }
    }
    result
}