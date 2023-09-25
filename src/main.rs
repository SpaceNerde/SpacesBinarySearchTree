use std::cmp::Ordering;
use std::fmt::Debug;
#[derive(Debug)]
pub struct Node<T>
    where
        T: Ord + Debug + Copy
{
    key: Option<T>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub fn new() -> Node<T> {
    Node {
        key: None,
        left: None,
        right: None,
    }
}

pub fn from(key: T) -> Node<T> {
    Node {
        key: Some(key),
        left: None,
        right: None,
    }
}

pub fn insert(&mut self, key: T) {
    match &self.key {
        Some(key) => {
            let target = match key.cmp(&key) {
                Ordering::Less => &mut self.left,
                Ordering::Equal => &mut self.left,
                Ordering::Greater => &mut self.right,
            };
            match target {
                Some(ref mut node) => {
                    node.insert(key)
                }
                None => {
                    let node: Node<T> = Node::from(key);
                    *target = Some(Box::new(node))
                }
            }
        }
        None => self.key = Some(key),
    }
}

pub fn minimum(&self) -> Option<&T> {
    match &self.left {
        Some(node) => node.minimum(),
        None => match &self.key {
            Some(key) => Some(&key),
            None => None,
        },
    }
}

pub fn maximum(&self) -> Option<&T> {
    match &self.right {
        Some(node) => node.maximum(),
        None => match &self.key {
            Some(key) => Some(&key),
            None => None,
        },
    }
}

pub fn search(&self, target: &T) -> bool {
    match &self.key {
        Some(key) => match &target.cmp(key) {
            Ordering::Equal => return true,
            Ordering::Less => match &self.left {
                Some(node) => node.search(target),
                None => false,
            },
            Ordering::Greater => match &self.right {
                Some(node) => node.search(target),
                None => false,
            },
        },
        None => return false,
    }
}

fn main() {
    let node1 = new();
    let node2 = new();

    println!("{:?}", node1);
    println!("{:?}", node2);
}