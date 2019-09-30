#[derive(Clone, Debug)]

pub enum Node<T> {
    Empty {},

    Root {
        hash: Vec<u8>,
        children: Vec<Box<Node<T>>>,
        content: Vec<T>,
    },
    Normal {
        hash: Vec<u8>,
        parent: Box<Node<T>>,
        children: Vec<Box<Node<T>>>,
        content: Vec<T>,
    },
}

impl<T> Node<T> {
    /// Create an empty tree
    pub fn empty() -> Self {
        Node::Empty {}
    }
}

pub fn test() {
    println!("this is a test");
}
