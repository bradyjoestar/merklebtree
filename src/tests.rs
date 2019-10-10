use crate::merklebtree::{MerkleBTree, Nodes};
use crate::node::Node;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Item {
    pub key: i32,
    pub value: String,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.key).cmp(&(other.key))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
pub struct Item2 {
    pub key: i32,
    pub value: i32,
}

impl PartialEq for Item2 {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for Item2 {}

impl Ord for Item2 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.key).cmp(&(other.key))
    }
}

impl PartialOrd for Item2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_btree_get_1() {
    let mut nodes_map: HashMap<i32, Node<Item>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);
    let mut testdata: Vec<Vec<(Item, bool)>> = Vec::new();

    let mut test_subdata1 = Vec::new();
    test_subdata1.push((
        Item {
            key: 0,
            value: String::from("m"),
        },
        false,
    ));
    testdata.push(test_subdata1);

    for i in 1..8 {
        let string = String::from_utf8(vec![(i + 96) as u8]).unwrap();
        let mut test_subdata = Vec::new();
        test_subdata.push((
            Item {
                key: i,
                value: string.clone(),
            },
            true,
        ));
        testdata.push(test_subdata);
        tree.put(
            Item {
                key: i,
                value: string.clone(), //'a'
            },
            &mut nodes,
        );
    }

    let mut test_subdata2 = Vec::new();
    test_subdata2.push((
        Item {
            key: 8,
            value: String::from("n"),
        },
        false,
    ));
    testdata.push(test_subdata2);

    for test_vec in testdata.iter() {
        for test_item in test_vec.iter() {
            println!("{:?}", test_item);
            let (value, found) = tree.get(test_item.0.clone(), &mut nodes);
            assert_eq!(value, test_item.0);
            assert_eq!(found, test_item.1);
        }
    }
}
