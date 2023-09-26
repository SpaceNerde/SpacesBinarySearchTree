use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<V> where V: Ord + Debug + Copy
{
    value: Option<V>,
    left: Option<Box<Node<V>>>,
    right: Option<Box<Node<V>>>,
}


pub fn new() -> Node<V> {
    Node {
        value: None,
        left: None,
        right: None,
    }
}

pub fn new_from(value: V) -> Node<V> {
    Node {
        value: Some(value),
        left: None,
        right: None,
    }
}


fn main() {
    let mut node = Node::new();
    println!("{:?}", node)
}