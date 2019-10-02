use crate::node::Node;
use core::borrow::{Borrow, BorrowMut};

#[derive(Clone, Debug)]
pub struct MerkleBTree<T>
where
    T: PartialEq + PartialOrd + Ord,
{
    pub root: Option<Box<Node<T>>>,
    size: u32, // Total number of keys in the tree
    m: u32,    // order (maximum number of children)
}

impl<T> MerkleBTree<T>
where
    T: PartialEq + PartialOrd + Ord,
{
    pub fn new_with(order: u32, value: T) -> Self {
        return MerkleBTree {
            root: None,
            size: 0,
            m: order,
        };
    }

    pub fn put(&mut self, value: T) -> () {
        match self.root.borrow_mut() {
            None => {
                self.root = Some(Box::new(Node::new_node(value)));
                println!("this is nil tree")
            }
            Some(t) => {
                t.put(value);
                //                t.borrow_mut().put(value);
                println!("this isn't nil tree");
            }
        }
    }

    pub fn get_content(&self) -> Option<&Vec<T>> {
        match &self.root {
            None => None,
            Some(t) => t.get_content(),
        }
    }
}

pub fn test() {
    println!("this is merklebtree");
}
