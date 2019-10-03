use crate::node;
use crate::node::is_leaf;
use crate::node::Node;
use core::borrow::{Borrow, BorrowMut};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct MerkleBTree<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    empty: bool,
    root: Box<Node<T>>,
    m: u32, // order (maximum number of children)
}

impl<T> MerkleBTree<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    pub fn new_empty(order: u32) -> Self {
        let mut tree = MerkleBTree {
            empty: true,
            root: Box::new(Node::new_empty()),
            m: order,
        };
        tree.root.root_flag = true;
        tree
    }

    pub fn new_with(order: u32, value: T) -> Self {
        println!("{:?}", value);
        let mut tree = MerkleBTree {
            empty: false,
            root: Box::new(Node::new_node(value)),
            m: order,
        };
        tree.root.root_flag = true;
        tree
    }

    pub fn put(&mut self, value: T) -> () {
        if self.empty {
            self.root = Box::new(Node::new_node(value));
            self.empty = false;
            self.root.root_flag = true;
        } else {
            let a = &mut self.root;
            node::insert(a, value, self.m);
        }
    }

    pub fn get_content(&self) -> Option<&Vec<T>> {
        if self.empty {
            None
        } else {
            self.root.get_content()
        }
    }
}
