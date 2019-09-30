#[derive(Clone, Debug)]
pub enum Node<T>
where
    T: PartialEq + PartialOrd + Ord,
{
    Empty {},

    Normal {
        parent: Option<Box<Node<T>>>,
        children: Vec<Box<Node<T>>>,
        content: Vec<T>,
    },
}

impl<T> Node<T>
where
    T: PartialEq + PartialOrd + Ord,
{
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
                let content_slice = content.as_slice();
                match content_slice.binary_search(&value) {
                    Ok(t) => println!("found,not insert"),
                    Err(e) => {
                        println!("not found,insert");
                        content.insert(e, value);
                    }
                }
                true
            }
        }
    }

    pub fn get_content(&self) -> Option<&Vec<T>> {
        match *self {
            Node::Empty {} => None,
            Node::Normal { ref content, .. } => Some(content),
        }
    }
}

pub fn test() {
    println!("this is a test");
}
