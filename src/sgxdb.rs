use crate::merklebtree::Nodes;
use crate::node::*;
use crate::traits::CalculateHash;
use std::fmt::Debug;

pub fn clone_insert<T>(
    insert_id: i32,
    value: T,
    order: u32,
    nodes: &mut Nodes<T>,
    subnodes: &mut Nodes<T>,
) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    if is_leaf(insert_id, nodes) {
        clone_insert_into_leaf(insert_id, value, order, nodes, subnodes)
    } else {
        clone_insert_into_internal(insert_id, value, order, nodes, subnodes)
    }
}

pub fn clone_insert_into_leaf<T>(
    insert_id: i32,
    value: T,
    order: u32,
    nodes: &mut Nodes<T>,
    subnodes: &mut Nodes<T>,
) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let node = nodes.nodes_map.get_mut(&insert_id).unwrap();

    let subnode_clone = node.clone();
    subnodes.nodes_map.insert(insert_id, subnode_clone);

    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => {
            node.content.remove(t);
            node.content.insert(t, value);
            recalculate_hash(nodes, insert_id);
            return false;
        }
        Err(e) => {
            node.content.insert(e, value);
            clone_split_node(insert_id, order, nodes, subnodes);
            return true;
        }
    }
}

pub fn clone_insert_into_internal<T>(
    insert_id: i32,
    value: T,
    order: u32,
    nodes: &mut Nodes<T>,
    subnodes: &mut Nodes<T>,
) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let node = nodes.nodes_map.get_mut(&insert_id).unwrap();

    let subnode_clone = node.clone();
    subnodes.nodes_map.insert(insert_id, subnode_clone);


    let content_slice = node.content.as_slice();

    match content_slice.binary_search(&value) {
        Ok(t) => {
            node.content.remove(t);
            node.content.insert(t, value);
            recalculate_hash(nodes, insert_id);
            return false;
        }
        Err(e) => {
            return clone_insert(
                *node.children_id.get(e).unwrap(),
                value,
                order,
                nodes,
                subnodes,
            );
        }
    }
}

pub fn clone_split_node<T>(split_id: i32, order: u32, nodes: &mut Nodes<T>, subnodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let node = nodes.nodes_map.get_mut(&split_id).unwrap();
    if !(node.content.len() > (order - 1) as usize) {
        recalculate_hash(nodes, split_id);
        return;
    } else {
        if node.root_flag {
            clone_split_root(split_id, order, nodes, subnodes)
        } else {
            clone_split_not_root(split_id, order, nodes, subnodes)
        }
    }
}

pub fn clone_split_root<T>(split_id: i32, order: u32, nodes: &mut Nodes<T>, subnodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let middle = middle(order);

    let mut left_node = Node::new_empty(nodes.next_id);
    let mut right_node = Node::new_empty(nodes.next_id + 1);
    let mut root_node = Node::new_empty(nodes.root_id);

    root_node.root_flag = true;
    let node = nodes.nodes_map.get_mut(&split_id).unwrap();

    root_node.content = node.content.split_off(middle as usize);
    right_node.content = root_node.content.split_off(1);
    left_node.content = node.content.clone();

    // Move children from the node to be split into left and right nodes
    if !(node.children_id.len() == 0) {
        right_node.children_id = node.children_id.split_off((middle + 1) as usize);
        left_node.children_id = node.children_id.clone();
        set_parent(&mut (left_node.children_id), left_node.node_id, nodes);
        set_parent(&mut (right_node.children_id), right_node.node_id, nodes);
    }

    right_node.parent_id = root_node.node_id;
    left_node.parent_id = root_node.node_id;
    root_node.children_id.push(left_node.node_id);
    root_node.children_id.push(right_node.node_id);

    let left_id = left_node.node_id;
    let right_id = right_node.node_id;
    let root_id = root_node.node_id;

    nodes.nodes_map.insert(nodes.next_id, left_node);
    nodes.nodes_map.insert(nodes.next_id + 1, right_node);
    nodes.nodes_map.insert(nodes.root_id, root_node);

    calculate_hash(left_id, nodes);
    calculate_hash(right_id, nodes);
    calculate_hash(root_id, nodes);

    nodes.next_id = nodes.next_id + 2;
    nodes.size = nodes.size + 2;
}

pub fn clone_split_not_root<T>(
    split_id: i32,
    order: u32,
    nodes: &mut Nodes<T>,
    subnodes: &mut Nodes<T>,
) where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let middle = middle(order);
    let mut node = nodes.nodes_map.remove(&split_id).unwrap();
    let parent_id = node.parent_id;
    let mut parent_node = nodes.nodes_map.remove(&parent_id).unwrap();

    //remove index of split_node in parent
    for i in 0..parent_node.children_id.len() {
        let find_id = *parent_node.children_id.get(i).unwrap();
        if find_id == split_id {
            parent_node.children_id.remove(i);
            break;
        }
    }

    let mut left_node = Node::new_empty(nodes.next_id);
    let mut right_node = Node::new_empty(nodes.next_id + 1);
    let mut medium_node = Node::new_empty(nodes.root_id);

    medium_node.content = node.content.split_off(middle as usize);
    right_node.content = medium_node.content.split_off(1);
    left_node.content = node.content.clone();
    left_node.parent_id = parent_id;
    right_node.parent_id = parent_id;

    if !(node.children_id.len() == 0) {
        right_node.children_id = node.children_id.split_off((middle + 1) as usize);
        left_node.children_id = node.children_id.clone();
        set_parent(&mut (left_node.children_id), left_node.node_id, nodes);
        set_parent(&mut (right_node.children_id), right_node.node_id, nodes);
    }

    // Insert middle key into parent
    let content_slice = parent_node.content.as_slice();
    let value = medium_node.content.remove(0);
    match content_slice.binary_search(&value) {
        Err(e) => {
            parent_node.content.insert(e, value);
            parent_node.children_id.insert(e, left_node.node_id);
            parent_node.children_id.insert(e + 1, right_node.node_id);

            let left_id = left_node.node_id;
            let right_id = right_node.node_id;
            let parent_id = parent_node.node_id;

            nodes.nodes_map.insert(parent_node.node_id, parent_node);
            nodes.nodes_map.insert(nodes.next_id, left_node);
            nodes.nodes_map.insert(nodes.next_id + 1, right_node);

            calculate_hash(left_id, nodes);
            calculate_hash(right_id, nodes);
            calculate_hash(parent_id, nodes);

            nodes.next_id = nodes.next_id + 2;
            nodes.size = nodes.size + 1;
            split_node(parent_id, order, nodes);
        }
        _ => {}
    }
}
