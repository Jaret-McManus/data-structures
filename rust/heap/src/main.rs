mod heap;
use heap::Heap;

fn main() {
    let mut h = Heap::<i32>::new();
    h.print_root();
    h.insert(100);
    h.print_root();
    h.insert(1);
    h.print_root();
    h.insert(2);
    h.print_root();
}
