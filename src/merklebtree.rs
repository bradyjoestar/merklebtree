use crate::node::Node;
use core::borrow::BorrowMut;

#[derive(Clone, Debug)]
pub struct MerkleBTree<T> {
    pub root: Box<Node<T>>,
    size: u32, // Total number of keys in the tree
    m: u32,    // order (maximum number of children)
}

impl<T> MerkleBTree<T> {
    pub fn new_with(order: u32, value: T) -> Self {
        return MerkleBTree {
            root: Box::new(Node::empty()),
            size: 0,
            m: order,
        };
    }

    pub fn put(&mut self, value: T) -> () {
        match self.root.borrow_mut() {
            Node::Empty {} => {
                println!("this is a empty node");
                self.root = Box::new(Node::new_node());
                self.root.put(value);
            }
            _ => {
                self.root.put(value);
            }
        }
    }

    pub fn get_content(&self) -> Option<&Vec<T>> {
        self.root.get_content()
    }
}

pub fn test() {
    println!("this is merklebtree");
}
