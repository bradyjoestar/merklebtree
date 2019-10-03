use crate::node;
use crate::node::is_leaf;
use crate::node::Node;
use core::borrow::{Borrow, BorrowMut};

#[derive(Clone, Debug)]
pub struct MerkleBTree<T>
where
    T: PartialEq + PartialOrd + Ord + Clone,
{
    empty: bool,
    root: Box<Node<T>>,
    m: u32, // order (maximum number of children)
}

impl<T> MerkleBTree<T>
where
    T: PartialEq + PartialOrd + Ord + Clone,
{
    pub fn new_with(order: u32, value: T) -> Self {
        return MerkleBTree {
            empty: true,
            root: Box::new(Node::new_empty()),
            m: order,
        };
    }

    pub fn put(&mut self, value: T) -> () {
        if self.empty {
            self.root = Box::new(Node::new_node(value));
            self.empty = false;
            println!("this is nil tree")
        } else {
            let a = &mut self.root;
            node::insert(a, value);
            println!("this isn't nil tree");
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

pub fn test() {
    println!("this is merklebtree");
}
