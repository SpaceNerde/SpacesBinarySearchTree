use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<T: Ord>
{
    key: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T: Ord> Node<T> {
    pub fn new(key: T) -> Node<T> {
        Node {
            key,
            left: None,
            right: None,
        }
    }

    pub fn insert(mut root: &mut Option<Box<Node<T>>>, key: T) -> Result<(), ()>{
        while let Some(ref mut node) = root{
            match key.cmp(&node.key) {
                Ordering::Equal => root = &mut node.left,
                Ordering::Less => root = &mut node.left,
                Ordering::Greater => root = &mut node.right,
            }
        }
        *root = Some(Box::new(Node::new(key)));

        Ok(())
    }

}

fn main() {
    let mut node = Node::new(10);
    println!("{:?}", node);


    println!("{:?}", Node::insert(&mut Some(Box::new(node)), 9));
}