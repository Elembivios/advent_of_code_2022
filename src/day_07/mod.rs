use crate::utils::tree::Node;
use std::{fmt, sync::Arc};

#[derive(Debug)]
struct Directory {
    name: String,
    size: Option<u64>,
    file_sizes: Vec<u64>
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Directory {
    fn new(name: impl Into<String>) -> Self {
        Directory {
            name: name.into(),
            size: None,
            file_sizes: vec![],
        }
    }
}

pub struct NoSpaceLeftOnDevice {
    root: Node<Directory>
}

impl crate::Advent for NoSpaceLeftOnDevice {
    fn new(data: &str) -> Self {

        let dir = Directory::new("/".to_string());
        let root: Node<Directory> = Node::new(dir);
        let mut current = Node {
            arc_ref: root.get_copy_of_internal_arc()
        };

        for l in data.lines().skip(1) {
            if l.starts_with("$") {
                match &l[2..4] {
                    "ls" => { continue; },
                    "cd" => {
                        match &l[5..] {
                            ".." => {
                                current = Node { arc_ref: current.get_parent().unwrap() };
                            },
                            name => {
                                let children = &current.get_copy_of_internal_arc().children;
                                let index = children.read().unwrap().iter().position(|c| {
                                    c.value.write().unwrap().name == name
                                });
                                let child = Node { 
                                    arc_ref: Arc::clone(&children.read().unwrap()[index.unwrap()])
                                };
                                current = child;
                            }
                        }
                    },
                    _ => panic!("Invalid line: {}", l)
                }
            } else {
                let (lhs, name) = l.split_once(" ").unwrap();
                match lhs {
                    "dir" => {                        
                        current.create_and_add_child(Directory::new(name));                        
                    },
                    num_str => {
                        let size: u64 = num_str.parse().unwrap();                     
                        current.arc_ref.value.write().unwrap().file_sizes.push(size);
                    }
                }
            }
        };
        NoSpaceLeftOnDevice { root }
    }

    fn part_01(&self) -> String {
        let mut sub_100k_sizes_sum: u64 = 0;
        for node in self.root.inorder_iter() {            
            let sub_dirs_size: u64 = node.children.read().unwrap().iter().map(|n| {
                n.value.read().unwrap().size.unwrap()
            }).sum();
            let files_size: u64 = node.value.read().unwrap().file_sizes.iter().sum();
            let size_sum = sub_dirs_size + files_size;
            if size_sum < 100_000 {
                sub_100k_sizes_sum += size_sum
            }
            node.value.write().unwrap().size = Some(size_sum);
        }
        sub_100k_sizes_sum.to_string()
    }
    
    fn part_02(&self) -> String {

        let root_size = self.root.value.read().unwrap().size.unwrap();        
        let total_available: u64 = 70_000_000;
        let required_size: u64 = 30_000_000;
        let size_left = total_available - root_size;
        let size_to_delete = required_size - size_left;

        let mut nodes: Vec<u64> = self.root.inorder_iter().filter_map(|n| {
            let size = n.value.read().unwrap().size.unwrap();
            if size > size_to_delete {
                Some(size)
            } else {
                None
            }
        }).collect();
        nodes.sort_unstable();
        nodes[0].to_string()
    }
}