// My (not working) implementation using a tree, which didnt go well


use crate::utils::tree::{Node, NodeData};
use std::sync::Arc;
use std::cmp::Ordering;

pub struct DistressSignal {
    pairs: Vec<Vec<Node<Option<u32>>>>
}

impl crate::Advent for DistressSignal {
    fn new(data: &str) -> Self {
        let lines: Vec<&str> = data.lines().collect();
        let pairs: Vec<_> = lines
            .split(|l| l.is_empty())
            .map(|pair| {                
                pair.iter().map(|part| {
                    parse_tree(&part)
                }).collect::<Vec<Node<Option<u32>>>>()
            }).collect();        
        DistressSignal { 
            pairs
        }
    }

    fn part_01(&self) -> String {
        let mut inorder: Vec<usize> = vec![];
        for (i, pair) in self.pairs.iter().enumerate() {
            let res = compare_trees(&pair[0], &pair[1]);
            match res {
                Ok(_) => {
                    println!("{} -> Ok", i);
                    inorder.push(i + 1)
                },
                Err(e) => {
                    println!("{} -> Error: {:?}", i, e);
                }
            }
        }
        println!("Inorder: {:?}", inorder);
        inorder.iter().sum::<usize>().to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}

fn parse_tree(tree_str: &str) -> Node<Option<u32>> {
    let root: Node<Option<u32>> = Node::new(None);
    let mut current = Node { arc_ref: root.get_copy_of_internal_arc() };

    let mut current_digits: Vec<u32> = vec![];
    for char in tree_str.chars().skip(1) {
        match char {
            ',' => {
                if current_digits.len() != 0 {
                    let num: u32 = current_digits
                        .drain(..)
                        .rev()
                        .enumerate()
                        .fold(0, |acc, (i, digit)| {
                            acc + 10_u32.pow(i as u32) * digit
                        });   
                    let child = Node::new(Some(num));
                    current.add_child_and_update_its_parent(&child);                                    
                }
            },
            '[' => {
                // current.get_copy_of_internal_arc().value.write().unwrap().push(Item::L);

                let child = Node::new(None);
                current.add_child_and_update_its_parent(&child);
                current = child;
            },
            ']' => {
                if current_digits.len() != 0 {
                    let num: u32 = current_digits
                        .drain(..)
                        .rev()
                        .enumerate()
                        .fold(0, |acc, (i, digit)| {
                            acc + 10_u32.pow(i as u32) * digit
                        });   
                    let child = Node::new(Some(num));
                    current.add_child_and_update_its_parent(&child);
                }

                if let Some(parent) = current.get_parent() {
                    current = Node { arc_ref: parent };
                }                                
            },
            x => {
                current_digits.push(x.to_digit(10).unwrap())
            }
        }
    }
    root
}

#[derive(Debug)]
enum CompareError {
    LeftGreaterThanRight((u32, u32)),
    RightSideRanOutOfItems,
    RightSideRanOutOfInnerItems,
}

fn compare_trees(tree_01: &Node<Option<u32>>, tree_02: &Node<Option<u32>>) -> Result<(), CompareError> {
    let mut lhs_it = tree_01
        .inorder_iter().map(|(level, lhs_ref)| {
            (level, get_val(lhs_ref))
        });
    let mut rhs_it = tree_02
        .inorder_iter().map(|(level, rhs_ref)| {
            (level, get_val(rhs_ref))
        });
    let mut lhs_item = lhs_it.next();
    let mut rhs_item = rhs_it.next();
    'zip: loop {
        println!("\t{lhs_item:?} -- {rhs_item:?}");
        match (lhs_item, rhs_item) {
            (Some((lhs_l, lhs_i)), Some((rhs_l, rhs_i))) => {                                      
                match (lhs_i, rhs_i) {   
                    (None, None) => {
                        if lhs_l < rhs_l {
                            println!("\tLeft side ran out of items.");
                            break 'zip;
                        } else if rhs_l < lhs_l {                            
                            println!("\tRight side ran out of items.");
                            return Err(CompareError::RightSideRanOutOfItems); 
                        }
                        lhs_item = lhs_it.next();
                        rhs_item = rhs_it.next();
                        continue 'zip;
                    },                         
                    (Some(lhs), Some(rhs)) => {
                        if lhs_l < rhs_l {
                            println!("\tRight side ran out of items.");
                            return Err(CompareError::RightSideRanOutOfItems);                
                        } else if rhs_l < lhs_l {
                            println!("\tLeft side ran out of items.");
                            break 'zip;
                        }

                        match lhs.cmp(&rhs) {
                            Ordering::Less => break 'zip,
                            Ordering::Greater => {
                                return Err(CompareError::LeftGreaterThanRight((lhs, rhs)));
                            },
                            Ordering::Equal => {                                        
                                lhs_item = lhs_it.next();
                                rhs_item = rhs_it.next();
                            }
                        };
                    },
                    (Some(_), None) => {
                        println!("\tMismatched types, going down on rhs ^^");                  
                        if let Some((next_rhs_l, next_rhs_i)) = rhs_it.next() {
                            if next_rhs_l <= rhs_l {
                                return Err(CompareError::RightSideRanOutOfInnerItems);
                            }
                            rhs_item = Some((rhs_l, next_rhs_i));
                        } else {
                            rhs_item = None
                        }
                        continue 'zip;
                    },
                    (None, Some(_)) => {
                        println!("\tMismatched types, going down on lhs");
                        if let Some((next_lhs_l, next_lhs_i)) = lhs_it.next() {
                            if next_lhs_l <= lhs_l {
                                println!("\tLeft side ran out of inner items.");
                                break 'zip;
                            }
                            lhs_item = Some((lhs_l, next_lhs_i));
                        } else {
                            lhs_item = None
                        }
                        continue 'zip;
                    },
                    
                }
            },
            (Some(_), None) => {
                return Err(CompareError::RightSideRanOutOfItems);
            },
            (None, _) => {
                break 'zip;
            }
        }
    }    
    
    Ok(())
}

fn get_val(node_ref: Arc<NodeData<Option<u32>>>) -> Option<u32> {
    *node_ref.value.read().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let t1 = parse_tree("[1,1,3,1,1]");
        let t2 = parse_tree("[1,1,5,1,1]");        
        assert_eq!(compare_trees(&t1, &t2).is_ok(), true);

        let t1 = parse_tree("[[1],[2,3,4]]");
        let t2 = parse_tree("[[1],4]");        
        assert_eq!(compare_trees(&t1, &t2).is_ok(), true);

        let t1 = parse_tree("[9]");
        let t2 = parse_tree("[[8,7,6]]");        
        let res = compare_trees(&t1, &t2);
        assert!(matches!(res, Err(CompareError::LeftGreaterThanRight(_))));
        
        let t1 = parse_tree("[[4,4],4,4]");
        let t2 = parse_tree("[[4,4],4,4,4]");  
        let res = compare_trees(&t1, &t2);
        assert_eq!(res.is_ok(), true);

        let t1 = parse_tree("[7,7,7,7]");
        let t2 = parse_tree("[7,7,7]");  
        let res = compare_trees(&t1, &t2);
        assert!(matches!(res, Err(CompareError::RightSideRanOutOfItems)));

        let t1 = parse_tree("[]");
        let t2 = parse_tree("[3]");  
        let res = compare_trees(&t1, &t2);
        assert_eq!(res.is_ok(), true);

        let t1 = parse_tree("[[[]]]");
        let t2 = parse_tree("[[]]");  
        let res = compare_trees(&t1, &t2);
        assert!(matches!(res, Err(CompareError::RightSideRanOutOfItems)));

        let t1 = parse_tree("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let t2 = parse_tree("[1,[2,[3,[4,[5,6,0]]]],8,9]");  
        let res = compare_trees(&t1, &t2);
        assert!(matches!(res, Err(CompareError::LeftGreaterThanRight(_))));

    }

    #[test]
    fn test_some_real() {
        println!("1");
        let t1 = parse_tree("[[],[],[[],10,[[7,0,1,1,10],9,6],[1,[4,9,1],6,[4,6,0]],[0,3,0]],[1,[10,[7,4,3,4],[]],[2,2],7,[[10],5,3]]]");
        let t2 = parse_tree("[[8,1,[2,3,3,[6,7,7,2,6]],[[8,10,1]],[9]],[1,7,[7,[3,6],7,7,10]]]");  
        let res = compare_trees(&t1, &t2);
        assert_eq!(res.is_ok(), true);

        println!("2");
        let t1 = parse_tree("[[[[3,0,4,4],9,2],8,[[1],3,[2,0,9,3],[5,3,6,4,5]]],[],[]]");
        let t2 = parse_tree("[[[],[]],[[[8,4],[5,5,9,9],[1,9,1,8],0]]]");  
        let res = compare_trees(&t1, &t2);
        assert!(matches!(res, Err(CompareError::RightSideRanOutOfItems)));

        println!("3");
        let t1 = parse_tree("[[5,[4,[8,3,4],1,1,[3,8,9,4,0]],4]]");
        let t2 = parse_tree("[[7,1,[[9,0,5,5,9],[5,0,3],[0,5,5,10,0],3,7],1],[9,[[1,6,3,2,2],7,[8,9],[7,7,2,10]],[],[]],[[8,0,[0],5],[[3]],3,[[1,0,0,9],[10,7,0,10]]],[[],8,2,3,[[10],[10,6,10],0,9]]]");
        let res = compare_trees(&t1, &t2);
        assert_eq!(res.is_ok(), true);

        println!("4");
        let t1 = parse_tree("[[4,8,6,8]]");
        let t2 = parse_tree("[[[3,[],[4,7,4,4],1],1,[6,[5],[1,1,6,8]],3,10],[2,[3,6,[],6],10]]");
        let res = compare_trees(&t1, &t2);
        assert!(matches!(res, Err(CompareError::LeftGreaterThanRight(_))));

        println!("5");
        let t1 = parse_tree("[[1],[[0],5]]");
        let t2 = parse_tree("[[10,[[2],10,[6,4,0],[],9]],[4,9,[[9,6,10,7]]]]");
        let res = compare_trees(&t1, &t2);
        assert_eq!(res.is_ok(), true);

        println!("6");
        let t1 = parse_tree("[[8,[[4,1,1,4],[6,10,6,6],7,6,[9,9,1]],[[0,9],3]],[1,[5],5,[]]]");
        let t2 = parse_tree("[[],[[]]]");
        let res = compare_trees(&t1, &t2);
        assert!(matches!(res, Err(CompareError::RightSideRanOutOfItems)));

    }

    #[test]
    fn test_wierd_iter() {
        let t = parse_tree("[[[],[]],[[[8,4],[5,5,9,9],[1,9,1,8],0]]]");
        let res: Vec<_> = t.inorder_iter().map(|(l, n)| {
            (l, *n.value.read().unwrap())
        }).collect();
        assert_eq!(res, vec![
            (0, None),
            (1, None), (2, None), (2, None),
            (1, None), (2, None),
            (3, None), (4, Some(8)), (4, Some(4)),
            (3, None), (4, Some(5)), (4, Some(5)), (4, Some(9)), (4, Some(9)),
            (3, None), (4, Some(1)), (4, Some(9)), (4, Some(1)), (4, Some(8)),
            (3, Some(0))
        ])
        
    }

}