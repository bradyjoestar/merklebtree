use crate::tree::Node;

#[derive(Clone, Debug)]
pub struct MerkleBTree<T> {
    root: Box<Node<T>>,
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
}

pub fn test() {
    println!("this is merklebtree");
}
