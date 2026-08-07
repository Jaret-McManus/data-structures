mod node;
use node::Node;

pub struct Heap<T: Ord> {
    root: Option<Node<T>>
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Heap<T> {
        return Heap { root: None }
    }

    pub fn from(value: T) -> Heap<T> {
        let node = Node::<T>::new(value);
        return Heap { root: Some(node) };
    }
}
