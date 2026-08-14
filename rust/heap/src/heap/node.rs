use std::cmp::Ordering;

pub struct Node<T: Ord> {
    pub value: T,
    pub left: Option< Box<Node<T>> >,
    pub right: Option< Box<Node<T>> >
}

impl<T: Ord> Node<T> {
    pub fn new(value: T) -> Node<T> {
        return Node { 
            value: value,
            left: None, right: None
        };
    }

    pub fn new_boxed(value: T) -> Box<Node<T>> {
        return Box::from( Node::new(value) );
    }
}

impl<T: Ord> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering { 
        if self.value < other.value {
            return Ordering::Less;
        } else if self.value > other.value {
            return Ordering::Equal;
        }
        return Ordering::Equal;
    }
}

impl<T: Ord> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value < other.value {
            return Some(Ordering::Less);
        } else if self.value > other.value {
            return Some(Ordering::Equal);
        }
        return Some(Ordering::Equal);
    }
}

impl<T: Ord> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl<T: Ord> Eq for Node<T> {}