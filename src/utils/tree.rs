use core::fmt::Debug;
use std::fmt::{self, Display};
use std::ops::Deref;
use std::sync::{Arc, RwLock, Weak};

type NodeDataRef<T> = Arc<NodeData<T>>;
type WeakNodeRef<T> = Weak<NodeData<T>>;
type Parent<T> = RwLock<WeakNodeRef<T>>;
type Child<T> = NodeDataRef<T>;
type Children<T> = RwLock<Vec<Child<T>>>;

pub struct NodeData<T>
where 
    T: Display,
{
    pub value: RwLock<T>,
    pub parent: Parent<T>,
    pub children: Children<T>
}

impl<T> Debug for NodeData<T>
where
    T: Debug + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parent_msg = String::new();
        if let Some(parent) = self.parent.read().unwrap().upgrade() {
            parent_msg.push_str(format!("📦 {}", parent.value.read().unwrap()).as_str());            
        } else {
            parent_msg.push_str("🚫 None");
        }
        f.debug_struct("Node")
            .field("value", &self.value)
            .field("parent", &parent_msg)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug)]
pub struct Node<T: Display> {
    pub arc_ref: NodeDataRef<T>,
}

impl<T> Deref for Node<T>
where 
    T: Display
{
    type Target = NodeData<T>;

    fn deref(&self) -> &Self::Target {
        &self.arc_ref
    }
}

impl<T> Node<T>
where
    T: Display + Debug,
{
    pub fn new(value: T) -> Node<T> {
        let new_node = NodeData {
            value: RwLock::new(value),
            parent: RwLock::new(Weak::new()),
            children: RwLock::new(Vec::new())
        };

        let arc_ref = Arc::new(new_node);
        Node { arc_ref }
    }

    pub fn get_copy_of_internal_arc(self: &Self) -> NodeDataRef<T> {
        Arc::clone(&self.arc_ref)
    }

    pub fn create_and_add_child(&self, value: T) -> NodeDataRef<T> {
        let new_child = Node::new(value);
        self.add_child_and_update_its_parent(&new_child);
        new_child.get_copy_of_internal_arc()
    }

    pub fn add_child_and_update_its_parent(&self, child: &Node<T>) {
        {
            let mut my_children = self.arc_ref.children.write().unwrap();
            my_children.push(child.get_copy_of_internal_arc());
        }

        {
            let mut childs_parent = child.arc_ref.parent.write().unwrap();
            *childs_parent = Arc::downgrade(&self.get_copy_of_internal_arc());
        }
    }

    pub fn get_parent(&self) -> Option<NodeDataRef<T>> {
        let my_parent_weak = self.arc_ref.parent.read().unwrap();
        if let Some(my_parent_arc_ref) = my_parent_weak.upgrade() {
            Some(my_parent_arc_ref)
        } else {
            None
        }
    }    

    pub fn inorder_iter(&self) -> InOrderIterator<T> {
        let node = Node { arc_ref: self.get_copy_of_internal_arc() };
        InOrderIterator::new(node)
    }
}


pub struct InOrderIterator<T>
where
    T: Display
{
    current: Node<T>,
    queue: Vec<usize> // Number of children to check for each node down the tree
}

impl<T> InOrderIterator<T>
where
    T: Display
{
    fn new(node: Node<T>) -> Self {
        InOrderIterator { current: node, queue: vec![0] }
    }
}

impl<T> Iterator for InOrderIterator<T>
where
    T: Debug + Display
{
    type Item = NodeDataRef<T>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let num_children = self.current.children.read().unwrap().len();
        match (self.current.get_parent(), self.queue.pop()) {
            (_, None) => None,
            (None, Some(i)) if i >= num_children => {
                Some(self.current.get_copy_of_internal_arc())
            },
            (Some(parent), Some(i)) if i >= num_children => {
                let current = self.current.get_copy_of_internal_arc();
                self.current = Node { arc_ref: parent };
                Some(current)
            },
            (_, Some(i)) => {                
                let next_child = Arc::clone(&self.current.children.read().unwrap()[i]);
                self.current = Node { arc_ref: next_child};
                self.queue.push(i + 1);
                self.queue.push(0);
                self.next()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;


    #[test]
    fn test_tree_low_level_node_manipulation() {
        let child_node = Node::new(3);

        {
            let parent_node = Node::new(5);
            parent_node.add_child_and_update_its_parent(&child_node);

            assert_eq!(parent_node.children.read().unwrap().len(), 1);
            assert!(parent_node.get_parent().is_none());
            assert_eq!(*parent_node.value.read().unwrap(), 5);
            assert_eq!(Arc::weak_count(&parent_node.arc_ref), 1);

            println!("{}: {:#?}", "[parent_node]", parent_node);
            println!("{}: {:#?}", "[child_node]", child_node);

            assert_eq!(Arc::strong_count(&child_node.get_copy_of_internal_arc()), 3);
            assert_eq!(Arc::weak_count(&child_node.get_copy_of_internal_arc()), 0);

            assert_eq!(Arc::strong_count(&parent_node.get_copy_of_internal_arc()), 2);
            assert_eq!(Arc::weak_count(&parent_node.get_copy_of_internal_arc()), 1);

            assert!(child_node.get_parent().is_some());
            assert_eq!(*child_node.get_copy_of_internal_arc().value.read().unwrap(), 3);
        } // Parent is dropped here

        // Child now has no parent, it's an orphan
        assert!(child_node.get_parent().is_none());
        assert_eq!(*child_node.get_copy_of_internal_arc().value.read().unwrap(), 3);
        
        assert_eq!(Arc::strong_count(&child_node.get_copy_of_internal_arc()), 2);
        assert_eq!(Arc::weak_count(&child_node.get_copy_of_internal_arc()), 0);
    }

    #[test]
    fn tree_simple_api() {
        let root_node = Node::new(5);
        assert_eq!(*root_node.get_copy_of_internal_arc().value.read().unwrap(), 5);

        {
            // In the following line 'Node' is returned by 'create_and_add_child()'.
            // Instead a ref ('Arc') to the underlying 'NodeData' is returned

            let child_node_data_ref = root_node.create_and_add_child(3);
            let child_val = *child_node_data_ref.value.read().unwrap();
            assert_eq!(child_val, 3);
            assert_eq!(
                root_node.get_copy_of_internal_arc().children.read().unwrap().len(),
                1
            );
            assert_eq!(
                child_val, 
                *root_node.get_copy_of_internal_arc().children.read().unwrap()[0].value.read().unwrap()
            )
        }
        println!("{}: {:#?}", "[tree]", root_node);        
    }

    #[test]
    fn test_inorder_iter() {
        let root_node = Node::new(1);
        root_node.create_and_add_child(10);
        root_node.create_and_add_child(20);
        let child = Node { arc_ref: root_node.create_and_add_child(30)};
        child.create_and_add_child(100);
        child.create_and_add_child(200);
        child.create_and_add_child(300);

        //         1
        //      /  |   \
        //    10  20   30
        //            / | \
        //         100 200 300

        let res: Vec<_> = root_node.inorder_iter().map(|child| *child.value.read().unwrap() ).collect();
        assert_eq!(res, vec![10, 20, 100, 200, 300, 30, 1]);
    }

    #[test]
    fn test_inorder_iter_02() {
        let root_node = Node::new(1);

        let child_01 = Node { arc_ref: root_node.create_and_add_child(10) };
        let sub_child_01 = Node { arc_ref: child_01.create_and_add_child(100)};
        sub_child_01.create_and_add_child(1000);
        
        root_node.create_and_add_child(20);
        let child = Node { arc_ref: root_node.create_and_add_child(30)};
        let sub_child = Node {arc_ref: child.create_and_add_child(200)};
        sub_child.create_and_add_child(2000);
        sub_child.create_and_add_child(3000);
        sub_child.create_and_add_child(4000);

        //         1
        //      /  |   \
        //    10  20   30
        //    /        |
        //  100       200  
        //   |      /  |  \
        //  1k    2k  3k   4k

        let res: Vec<_> = root_node.inorder_iter().map(|child| *child.value.read().unwrap() ).collect();
        assert_eq!(res, vec![1000, 100, 10, 20, 2000, 3000, 4000, 200, 30, 1]);
    }

}