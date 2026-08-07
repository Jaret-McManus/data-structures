struct Node<T> {
    value: T,
    left: Option<Node<T>>
    right: Option<Node<T>>
}


struct Heap<T> {
    root: Option<Node<T>>
}