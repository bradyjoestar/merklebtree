use crate::merklebtree::Nodes;
use std::fmt::Debug;

pub enum position {
    begin,
    between,
    end,
}

pub struct btree_iterator<'a, T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    pub nodes: &'a Nodes<T>,

    pub position: position,
}

pub fn new_btree_iterator<T>(nodes: &Nodes<T>, position: position) -> btree_iterator<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    btree_iterator { nodes, position }
}

pub fn next<T>(btree_iterator: btree_iterator<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    match btree_iterator.position {
        position::end => {
            println!("end");
        }
        position::begin => {
            println!("begin");
        }
        position::between => {
            println!("between");
        }
    }
    true
}
