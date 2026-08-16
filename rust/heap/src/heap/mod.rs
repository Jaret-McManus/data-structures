mod node;

use std::fmt::Display;

use node::Node;

pub struct Heap<T: Ord> {
    root: Option< Box<Node<T>> >
}

enum NodeDirection {
    Left, Right
}

impl<T: Ord + Display> Heap<T> {
    pub fn new() -> Heap<T> {
        return Heap { root: None }
    }

    pub fn from(value: T) -> Heap<T> {
        let boxed_node = Node::<T>::new_boxed(value);
        return Heap { root: Some(boxed_node) };
    }

    
    pub fn insert_recursive(&mut self, possible_curr_node: &mut Option< Box<Node<T>> >, value: T) -> () {
        match possible_curr_node {
            None => *possible_curr_node = Some(Node::new_boxed(value)),
            Some(curr_node) => {
                if value < curr_node.value {
                    self.insert_recursive(&mut curr_node.left, value);
                } else {
                    self.insert_recursive(&mut curr_node.right, value);
                }
            }
        };
    }

    pub fn insert(&mut self, value: T) -> () {
        let root: &mut Box<Node<T>>;
        match &mut self.root {
            None => { Heap::create_root(self, value); return; }
            Some(heap_root) => root = heap_root
        }

        let (parent, direction) = Heap::find_potential_parent(root, &value);
        let node_optional = Some(Node::new_boxed(value));
        match direction {
            NodeDirection::Left => parent.left = node_optional,
            NodeDirection::Right => parent.right = node_optional,
        }
        
    }

    fn create_root(heap: &mut Heap<T>, value: T) -> () {
        heap.root = Some(Node::new_boxed(value));
    }

    fn find_potential_parent<'a>(root: &'a mut Box<Node<T>>, value: &T) -> (&'a mut Box<Node<T>>, NodeDirection) {
        let mut depth = 50;
        let mut curr_node: &'a mut Box<Node<T>> = root;
        while depth >= 0 {
            if value < &curr_node.value  {
                if curr_node.left.is_none() {
                    return (curr_node, NodeDirection::Left);
                } else {
                    curr_node = curr_node.left.as_mut().unwrap();
                }
            } else {
                if curr_node.right.is_none() {
                    return (curr_node, NodeDirection::Right);
                } else {
                    curr_node = curr_node.right.as_mut().unwrap();
                }
            }

            depth -= 1;
        };

        panic!("No possible place on heap!")
    }

    pub fn print_root(&self) -> () {
        match &self.root {
            None => println!("None"),
            Some(node) => println!("Box<Node<{}>>", node.value)
        }
    }

    // pub fn print_path(node_path: Vec<NodeDirection>) {
    //     if node_path.is_empty() {
            
    //     }
    // }

    // fn print_optional_node() => 
}
