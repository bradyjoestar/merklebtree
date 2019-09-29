use crate::tree::Node;

#[derive(Clone, Debug)]
pub struct MerkleBTree<T> {
    root: Box<Node<T>>,
    size: u32, // Total number of keys in the tree
    m: u32,    // order (maximum number of children)
}

pub fn test() {
    println!("this is merklebtree");
}
