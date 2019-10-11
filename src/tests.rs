use crate::merklebtree::{MerkleBTree, Nodes};
use crate::node::Node;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

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
        content_size: 0,
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

#[test]
fn test_btree_get_2() {
    let mut nodes_map: HashMap<i32, Node<Item>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        content_size: 0,
        root_id: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);
    let mut testdata: Vec<Vec<(Item, bool)>> = Vec::new();

    tree.put(
        Item {
            key: 7,
            value: String::from("g"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 9,
            value: String::from("i"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 10,
            value: String::from("j"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 6,
            value: String::from("f"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 3,
            value: String::from("c"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 4,
            value: String::from("d"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 5,
            value: String::from("e"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 8,
            value: String::from("h"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 2,
            value: String::from("b"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 1,
            value: String::from("a"),
        },
        &mut nodes,
    );

    let mut test_subdata1 = Vec::new();
    test_subdata1.push((
        Item {
            key: 0,
            value: String::from("m"),
        },
        false,
    ));
    testdata.push(test_subdata1);

    for i in 1..11 {
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
    }

    let mut test_subdata2 = Vec::new();
    test_subdata2.push((
        Item {
            key: 11,
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

#[test]
fn test_btree_put_1() {
    let mut nodes_map: HashMap<i32, Node<Item2>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);
    assertValidTree(&nodes, 0);

    tree.put(Item2 { key: 1, value: 0 }, &mut nodes);
    assertValidTree(&nodes, 1);
    assertValidTreeNode(&vec![0], 1, 0, &vec![1], false, &nodes);

    tree.put(Item2 { key: 2, value: 1 }, &mut nodes);
    assertValidTree(&nodes, 2);
    assertValidTreeNode(&vec![0], 2, 0, &vec![1, 2], false, &nodes);

    tree.put(Item2 { key: 3, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 3);
    assertValidTreeNode(&vec![0], 1, 2, &vec![2], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 1, 0, &vec![3], true, &nodes);


    tree.put(Item2 { key: 4, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 4);
    assertValidTreeNode(&vec![0], 1, 2, &vec![2], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 2, 0, &vec![3,4], true, &nodes);

    tree.put(Item2 { key: 5, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 5);
    assertValidTreeNode(&vec![0], 2, 3, &vec![2,4], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 1, 0, &vec![3], true, &nodes);
    assertValidTreeNode(&vec![0, 2], 1, 0, &vec![5], true, &nodes);

    tree.put(Item2 { key: 6, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 6);
    assertValidTreeNode(&vec![0], 2, 3, &vec![2,4], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 1, 0, &vec![3], true, &nodes);
    assertValidTreeNode(&vec![0, 2], 2, 0, &vec![5,6], true, &nodes);


    tree.put(Item2 { key: 7, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 7);
    assertValidTreeNode(&vec![0], 1, 2, &vec![4], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 1, 2, &vec![6], true, &nodes);
    assertValidTreeNode(&vec![0, 0,0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNode(&vec![0, 0,1], 1, 0, &vec![3], true, &nodes);
    assertValidTreeNode(&vec![0, 1,0], 1, 0, &vec![5], true, &nodes);
    assertValidTreeNode(&vec![0, 1,1], 1, 0, &vec![7], true, &nodes);
}

fn assertValidTree<T>(nodes: &Nodes<T>, expectedSize: i32)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let (actualValue, expectedValue) = (nodes.content_size, expectedSize);
    if actualValue != expectedValue as i32 {
        panic!(
            "Got {} expected {} for content size",
            actualValue, expectedValue
        );
    }
}

fn assertValidTreeNode(
    branch: &Vec<i32>, //from root  i.e vec![0,1,2] //0 replace root
    expectedContents: i32,
    expectedChildren: i32,
    keys: &Vec<i32>,
    hasParent: bool,
    nodes: &Nodes<Item2>,
) {
    let node_id = find_nodeid_by_branch(branch, nodes);
    let node = nodes.nodes_map.get(&node_id).unwrap();
    let actualValue = node.parent_id != -1;
    if actualValue != hasParent {
        panic!("Got {} expected {} for hasParent", actualValue, hasParent);
    }
    let actualValue = node.content.len();
    if actualValue != expectedContents as usize {
        panic!(
            "Got {} expected {} for contents size",
            actualValue, expectedContents
        );
    }
    let actualValue = node.children_id.len();
    if actualValue != expectedChildren as usize {
        panic!(
            "Got {} expected {} for contents size",
            actualValue, expectedChildren
        );
    }

    let mut loop_time = 0;
    for i in keys.iter() {
        let actual_vale = node.content.get(loop_time).unwrap();
        println!("{:?}", actual_vale);
        if actual_vale.key != *i {
            panic!("Got {} expected {} for for Key", actual_vale.key, *i);
        }
        loop_time = loop_time + 1;
    }
}

fn find_nodeid_by_branch<T>(branch: &Vec<i32>, nodes: &Nodes<T>) -> i32
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let root_id = *branch.get(0).unwrap();
    let mut node = nodes.nodes_map.get(&root_id).unwrap();
    let mut node_id = 0;
    let mut iter_time = 0;
    for i in branch.iter() {
        if iter_time == 0 {
        } else {
            node_id = *node.children_id.get(*i as usize).unwrap();
            node = nodes.nodes_map.get(&node_id).unwrap();
        }
        iter_time = iter_time + 1;
    }
    node_id
}
