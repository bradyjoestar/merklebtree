#[derive(Clone, Debug)]
pub struct Node<T> {
    hash: Vec<u8>,
    parent: Box<Tree<T>>,
    children: Vec<Box<Tree<T>>>,
    content: Vec<T>,
}

pub fn test() {
    println!("this is a test");
}
