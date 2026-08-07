pub struct Node<T: Ord> {
    value: T,
    left: Option< Box<Node<T>> >,
    right: Option< Box<Node<T>> >
}

impl<T: Ord> Node<T> {
    pub fn new(value: T) -> Node<T> {
        return Node { 
            value: value,
            left: None, right: None
        };
    }
}

// impl<T: Ord> Ord for Node<T> {
//     fn cmp(&self, _: &Self) -> Ordering { 
//         return Ordering::Greater;
//     }
// }