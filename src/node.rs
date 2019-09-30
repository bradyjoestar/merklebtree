#[derive(Clone, Debug)]
pub enum Node<T> {
    Empty {},

    Normal {
        parent: Option<Box<Node<T>>>,
        children: Vec<Box<Node<T>>>,
        content: Vec<T>,
    },
}

impl<T> Node<T> {
    /// Create an empty tree
    pub fn empty() -> Self {
        Node::Empty {}
    }

    pub fn new_node() -> Self {
        Node::Normal {
            parent: None,
            children: vec![],
            content: vec![],
        }
    }

    pub fn put(&mut self, value: T) -> bool {
        match *self {
            Node::Empty {} => false,
            Node::Normal {
                ref mut content, ..
            } => {
                println!("-------insert data in content-------");
                content.push(value);
                true
            }
        }
    }

    pub fn get_content(&self) -> Option<&Vec<T>>{
        match *self{
            Node::Empty {} => None,
            Node::Normal {ref content,..} =>{
                Some(content)
            }
        }
    }
}

pub fn test() {
    println!("this is a test");
}
