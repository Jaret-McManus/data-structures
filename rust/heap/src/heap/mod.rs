mod node;

use node::Node;

pub struct Heap<T: Ord> {
    root: Option< Box<Node<T>> >
}

enum NodeDirection {
    Left, Right
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Heap<T> {
        return Heap { root: None }
    }

    pub fn from(value: T) -> Heap<T> {
        let boxed_node = Node::<T>::new_boxed(value);
        return Heap { root: Some(boxed_node) };
    }

    pub fn insert(mut self, value: T) -> () {
        let mut root: Box<Node<T>>;
        match self.root {
            None => { Heap::create_root(&mut self, value); return; }
            Some(heap_root) => root = heap_root
        }
        
        let (mut parent, direction) = Heap::find_potential_parent(&mut root, &value);
        let node_optional = Some(Node::new_boxed(value));
        match direction {
            NodeDirection::Left => parent.left = node_optional,
            NodeDirection::Right => parent.right = node_optional,
        }
        
    }

    fn create_root(heap: &mut Heap<T>, value: T) -> () {
        heap.root = Some(Node::new_boxed(value));
    }

    fn find_potential_parent(curr_boxed_node: &mut Box<Node<T>>, value: &T) -> (Node<T>, NodeDirection) {
        loop {
            if value < &curr_boxed_node.value {
                todo!()
            } else {
                todo!()
            }

        }
    } 
}
