use crate::merklebtree::Nodes;
use crate::node::*;
use crate::traits::CalculateHash;
use ring::digest;
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
    if !subnodes.nodes_map.contains_key(&insert_id) {
        subnodes.nodes_map.insert(insert_id, subnode_clone);
    }

    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => {
            node.content.remove(t);
            node.content.insert(t, value);
            clone_recalculate_hash(nodes, insert_id, subnodes);
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
    if !subnodes.nodes_map.contains_key(&insert_id) {
        subnodes.nodes_map.insert(insert_id, subnode_clone);
    }

    let content_slice = node.content.as_slice();

    match content_slice.binary_search(&value) {
        Ok(t) => {
            node.content.remove(t);
            node.content.insert(t, value);
            clone_recalculate_hash(nodes, insert_id, subnodes);
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
        clone_recalculate_hash(nodes, split_id, subnodes);
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
        clone_set_parent(
            &mut (left_node.children_id),
            left_node.node_id,
            nodes,
            subnodes,
        );
        clone_set_parent(
            &mut (right_node.children_id),
            right_node.node_id,
            nodes,
            subnodes,
        );
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

    clone_calculate_hash(left_id, nodes, subnodes);
    clone_calculate_hash(right_id, nodes, subnodes);
    clone_calculate_hash(root_id, nodes, subnodes);

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
        clone_set_parent(
            &mut (left_node.children_id),
            left_node.node_id,
            nodes,
            subnodes,
        );
        clone_set_parent(
            &mut (right_node.children_id),
            right_node.node_id,
            nodes,
            subnodes,
        );
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

            clone_calculate_hash(left_id, nodes, subnodes);
            clone_calculate_hash(right_id, nodes, subnodes);
            clone_calculate_hash(parent_id, nodes, subnodes);

            nodes.next_id = nodes.next_id + 2;
            nodes.size = nodes.size + 1;
            clone_split_node(parent_id, order, nodes, subnodes);
        }
        _ => {}
    }
}

pub fn clone_calculate_hash<T>(node_id: i32, nodes: &mut Nodes<T>, subnodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let mut hash = String::new();
    let mut node = nodes.nodes_map.remove(&node_id).unwrap();

    if !subnodes.nodes_map.contains_key(&node.node_id) {
        subnodes.nodes_map.insert(node.node_id, node.clone());
    }

    for i in node.content.iter() {
        hash.push_str(i.calculate().as_str());
    }

    node.children_hash.clear();
    for i in node.children_id.iter() {
        let child_node = nodes.nodes_map.get(i).unwrap();
        if !subnodes.nodes_map.contains_key(&child_node.node_id) {
            subnodes
                .nodes_map
                .insert(child_node.node_id, child_node.clone());
        }

        hash.push_str(child_node.hash.as_str());
        node.children_hash.push(child_node.hash.clone());
    }
    node.hash = hex::encode(digest::digest(&digest::SHA256, hash.as_ref()));
    nodes.nodes_map.insert(node_id, node);
}

/// ReCalculateMerkleRoot update Merkleroot from node to root node.
pub fn clone_recalculate_hash<T>(nodes: &mut Nodes<T>, node_id: i32, subnodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let mut node = nodes.nodes_map.remove(&node_id).unwrap();
    if !subnodes.nodes_map.contains_key(&node.node_id) {
        subnodes.nodes_map.insert(node.node_id, node.clone());
    }

    if node.node_id == nodes.root_id {
        nodes.nodes_map.insert(node.node_id, node);
        return clone_calculate_hash(node_id, nodes, subnodes);
    } else {
        let parent_id = node.parent_id;
        nodes.nodes_map.insert(node.node_id, node);
        clone_calculate_hash(node_id, nodes, subnodes);
        return clone_recalculate_hash(nodes, parent_id, subnodes);
    }
}

pub fn clone_delete<T>(
    node_id: i32,
    index: i32,
    nodes: &mut Nodes<T>,
    subnodes: &mut Nodes<T>,
) -> ()
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let mut node = nodes.nodes_map.remove(&node_id).unwrap();

    //clone node
    if !subnodes.nodes_map.contains_key(&node.node_id) {
        subnodes.nodes_map.insert(node.node_id, node.clone());
    }

    // deleting from a leaf node
    if node.children_id.len() == 0 {
        let delete_item = node.content.remove(index as usize);
        nodes.nodes_map.insert(node_id, node);
        clone_rebalance(node_id, delete_item, nodes, subnodes);
        return;
    }
    // deleting from an internal node
    let left_largest_node_id = clone_right(
        *node.children_id.get_mut(index as usize).unwrap(),
        nodes,
        subnodes,
    ); // largest node in the left sub-tree (assumed to exist)
    let mut left_largest_node = nodes.nodes_map.remove(&left_largest_node_id).unwrap();
    let left_largest_content_index = left_largest_node.content.len() - 1;

    node.content.remove(index as usize);
    let delete_item = left_largest_node.content.remove(left_largest_content_index);
    let delete_item_clone = delete_item.clone();
    node.content.insert(index as usize, delete_item);

    nodes.nodes_map.insert(node_id, node);
    nodes
        .nodes_map
        .insert(left_largest_node_id, left_largest_node);

    clone_rebalance(left_largest_node_id, delete_item_clone, nodes, subnodes);
}

pub fn clone_rebalance<T>(
    node_id: i32,
    mut value: T,
    nodes: &mut Nodes<T>,
    subnodes: &mut Nodes<T>,
) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    if (node_id == -1) {
        return false;
    }

    let mut node = nodes.nodes_map.get(&node_id).unwrap();
    //clone node
    if !subnodes.nodes_map.contains_key(&node.node_id) {
        subnodes.nodes_map.insert(node.node_id, node.clone());
    }

    let parent_id = node.parent_id;
    // check if rebalancing is needed
    if node.content.len() >= min_contents(nodes) as usize {
        clone_recalculate_hash(nodes, node_id, subnodes);
        return false;
    }

    let (left_sibling_id, left_sibling_index) =
        clone_left_sibling(node_id, &value, nodes, subnodes);
    if left_sibling_id != -1 {
        let mut left_sibling_node = nodes.nodes_map.remove(&left_sibling_id).unwrap();
        let mut delete_node = nodes.nodes_map.remove(&node_id).unwrap();
        let mut parent_node = nodes.nodes_map.remove(&parent_id).unwrap();

        //clone node
        if !subnodes.nodes_map.contains_key(&left_sibling_node.node_id) {
            subnodes
                .nodes_map
                .insert(left_sibling_node.node_id, left_sibling_node.clone());
        }
        if !subnodes.nodes_map.contains_key(&delete_node.node_id) {
            subnodes
                .nodes_map
                .insert(delete_node.node_id, delete_node.clone());
        }
        if !subnodes.nodes_map.contains_key(&parent_node.node_id) {
            subnodes
                .nodes_map
                .insert(parent_node.node_id, parent_node.clone());
        }

        if left_sibling_node.content.len() > min_contents(nodes) as usize {
            let sibling_data = left_sibling_node.content.pop().unwrap();
            let parent_data = parent_node.content.remove((left_sibling_index) as usize);
            delete_node.content.insert(0, parent_data);
            parent_node
                .content
                .insert(left_sibling_index as usize, sibling_data);

            nodes.nodes_map.insert(left_sibling_id, left_sibling_node);
            clone_calculate_hash(left_sibling_id, nodes, subnodes);

            if !is_leaf(left_sibling_id, nodes) {
                let mut left_sibling_node = nodes.nodes_map.remove(&left_sibling_id).unwrap();
                //clone node
                if !subnodes.nodes_map.contains_key(&left_sibling_node.node_id) {
                    subnodes
                        .nodes_map
                        .insert(left_sibling_node.node_id, left_sibling_node.clone());
                }

                let left_sibling_left_most_child_id = left_sibling_node.children_id.pop().unwrap();
                let mut left_sibling_left_most_child_node = nodes
                    .nodes_map
                    .remove(&left_sibling_left_most_child_id)
                    .unwrap();

                //clone node
                if !subnodes
                    .nodes_map
                    .contains_key(&left_sibling_left_most_child_node.node_id)
                {
                    subnodes.nodes_map.insert(
                        left_sibling_left_most_child_node.node_id,
                        left_sibling_left_most_child_node.clone(),
                    );
                }

                left_sibling_left_most_child_node.parent_id = node_id;

                delete_node
                    .children_id
                    .insert(0, left_sibling_left_most_child_id);

                nodes.nodes_map.insert(left_sibling_id, left_sibling_node);
                nodes.nodes_map.insert(
                    left_sibling_left_most_child_id,
                    left_sibling_left_most_child_node,
                );
                clone_calculate_hash(left_sibling_id, nodes, subnodes);
            }
            nodes.nodes_map.insert(node_id, delete_node);
            nodes.nodes_map.insert(parent_id, parent_node);
            clone_calculate_hash(node_id, nodes, subnodes);
            clone_recalculate_hash(nodes, parent_id, subnodes);
            return true;
        }
        nodes.nodes_map.insert(parent_id, parent_node);
        nodes.nodes_map.insert(left_sibling_id, left_sibling_node);
        nodes.nodes_map.insert(node_id, delete_node);
    }

    let (right_sibling_id, right_sibling_index) =
        clone_right_sibling(node_id, &value, nodes, subnodes);
    if right_sibling_id != -1 {
        let mut right_sibling_node = nodes.nodes_map.remove(&right_sibling_id).unwrap();
        let mut delete_node = nodes.nodes_map.remove(&node_id).unwrap();
        let mut parent_node = nodes.nodes_map.remove(&parent_id).unwrap();

        //clone node
        if !subnodes.nodes_map.contains_key(&right_sibling_node.node_id) {
            subnodes
                .nodes_map
                .insert(right_sibling_node.node_id, right_sibling_node.clone());
        }
        if !subnodes.nodes_map.contains_key(&delete_node.node_id) {
            subnodes
                .nodes_map
                .insert(delete_node.node_id, delete_node.clone());
        }
        if !subnodes.nodes_map.contains_key(&parent_node.node_id) {
            subnodes
                .nodes_map
                .insert(parent_node.node_id, parent_node.clone());
        }

        if right_sibling_node.content.len() > min_contents(nodes) as usize {
            let sibling_data = right_sibling_node.content.remove(0);
            let parent_data = parent_node
                .content
                .remove((right_sibling_index - 1) as usize);
            delete_node.content.push(parent_data);
            parent_node
                .content
                .insert((right_sibling_index - 1) as usize, sibling_data);

            nodes.nodes_map.insert(right_sibling_id, right_sibling_node);
            clone_calculate_hash(right_sibling_id, nodes, subnodes);

            if !is_leaf(right_sibling_id, nodes) {
                let mut right_sibling_node = nodes.nodes_map.remove(&right_sibling_id).unwrap();
                //clone node
                if !subnodes.nodes_map.contains_key(&right_sibling_node.node_id) {
                    subnodes
                        .nodes_map
                        .insert(right_sibling_node.node_id, right_sibling_node.clone());
                }

                let right_sibling_left_most_child_id =
                    right_sibling_node.children_id.remove(0 as usize);
                let mut right_sibling_left_most_child_node = nodes
                    .nodes_map
                    .remove(&right_sibling_left_most_child_id)
                    .unwrap();
                //clone node
                if !subnodes
                    .nodes_map
                    .contains_key(&right_sibling_left_most_child_node.node_id)
                {
                    subnodes.nodes_map.insert(
                        right_sibling_left_most_child_node.node_id,
                        right_sibling_left_most_child_node.clone(),
                    );
                }

                right_sibling_left_most_child_node.parent_id = node_id;
                delete_node
                    .children_id
                    .push(right_sibling_left_most_child_id);
                nodes.nodes_map.insert(right_sibling_id, right_sibling_node);
                nodes.nodes_map.insert(
                    right_sibling_left_most_child_id,
                    right_sibling_left_most_child_node,
                );
                clone_calculate_hash(right_sibling_id, nodes, subnodes);
            }
            nodes.nodes_map.insert(parent_id, parent_node);
            nodes.nodes_map.insert(node_id, delete_node);
            clone_calculate_hash(node_id, nodes, subnodes);
            clone_recalculate_hash(nodes, parent_id, subnodes);
            return true;
        }
        nodes.nodes_map.insert(parent_id, parent_node);
        nodes.nodes_map.insert(right_sibling_id, right_sibling_node);
        nodes.nodes_map.insert(node_id, delete_node);
    }

    // merge with siblings
    if right_sibling_id != -1 {
        // merge with right sibling
        let mut delete_node = nodes.nodes_map.remove(&node_id).unwrap();
        let mut parent_node = nodes.nodes_map.remove(&parent_id).unwrap();
        let mut right_sibling_node = nodes.nodes_map.remove(&right_sibling_id).unwrap();

        //clone node
        if !subnodes.nodes_map.contains_key(&right_sibling_node.node_id) {
            subnodes
                .nodes_map
                .insert(right_sibling_node.node_id, right_sibling_node.clone());
        }
        if !subnodes.nodes_map.contains_key(&delete_node.node_id) {
            subnodes
                .nodes_map
                .insert(delete_node.node_id, delete_node.clone());
        }
        if !subnodes.nodes_map.contains_key(&parent_node.node_id) {
            subnodes
                .nodes_map
                .insert(parent_node.node_id, parent_node.clone());
        }

        let parent_data = parent_node
            .content
            .remove((right_sibling_index - 1) as usize);
        value = parent_data.clone();
        delete_node.content.push(parent_data);
        for i in 0..right_sibling_node.content.len() {
            let right_sibling_node_data = right_sibling_node.content.remove(0);
            delete_node.content.push(right_sibling_node_data);
        }

        clone_set_parent(
            &mut (right_sibling_node.children_id),
            node_id,
            nodes,
            subnodes,
        );
        for i in 0..right_sibling_node.children_id.len() {
            delete_node
                .children_id
                .push(right_sibling_node.children_id.remove(0));
        }
        parent_node.children_id.remove(right_sibling_index as usize);

        nodes.nodes_map.insert(parent_id, parent_node);
        nodes.nodes_map.insert(right_sibling_id, right_sibling_node);
        nodes.nodes_map.insert(node_id, delete_node);

        clone_calculate_hash(node_id, nodes, subnodes);
        clone_calculate_hash(right_sibling_id, nodes, subnodes);
        clone_calculate_hash(parent_id, nodes, subnodes);

        nodes.size = nodes.size - 1;
    } else if left_sibling_id != -1 {
        // merge with left sibling
        let mut delete_node = nodes.nodes_map.remove(&node_id).unwrap();
        let mut parent_node = nodes.nodes_map.remove(&parent_id).unwrap();
        let mut left_sibling_node = nodes.nodes_map.remove(&left_sibling_id).unwrap();

        //clone node
        if !subnodes.nodes_map.contains_key(&left_sibling_node.node_id) {
            subnodes
                .nodes_map
                .insert(left_sibling_node.node_id, left_sibling_node.clone());
        }
        if !subnodes.nodes_map.contains_key(&delete_node.node_id) {
            subnodes
                .nodes_map
                .insert(delete_node.node_id, delete_node.clone());
        }
        if !subnodes.nodes_map.contains_key(&parent_node.node_id) {
            subnodes
                .nodes_map
                .insert(parent_node.node_id, parent_node.clone());
        }

        let parent_data = parent_node.content.remove((left_sibling_index) as usize);
        value = parent_data.clone();
        delete_node.content.insert(0, parent_data);

        for i in 0..left_sibling_node.content.len() {
            delete_node
                .content
                .insert(0, left_sibling_node.content.pop().unwrap())
        }

        clone_set_parent(
            &mut (left_sibling_node.children_id),
            node_id,
            nodes,
            subnodes,
        );
        for i in 0..left_sibling_node.children_id.len() {
            delete_node
                .children_id
                .insert(0, left_sibling_node.children_id.pop().unwrap() as i32)
        }
        parent_node.children_id.remove(left_sibling_index as usize);

        nodes.nodes_map.insert(parent_id, parent_node);
        nodes.nodes_map.insert(left_sibling_id, left_sibling_node);
        nodes.nodes_map.insert(node_id, delete_node);

        clone_calculate_hash(node_id, nodes, subnodes);
        clone_calculate_hash(left_sibling_id, nodes, subnodes);
        clone_calculate_hash(parent_id, nodes, subnodes);

        nodes.size = nodes.size - 1;
    }

    if parent_id == -1 {
        return false;
    }
    // make the merged node the root if its parent was the root and the root is empty
    let parent_node = nodes.nodes_map.get(&parent_id).unwrap();
    if parent_id == nodes.root_id && parent_node.content.len() == 0 {
        let mut node = nodes.nodes_map.remove(&node_id).unwrap();
        node.parent_id = -1;
        node.node_id = 0;
        node.root_flag = true;
        clone_set_parent(&mut node.children_id, node.node_id, nodes, subnodes);
        nodes.nodes_map.remove(&parent_id);
        nodes.nodes_map.insert(parent_id, node);
        clone_calculate_hash(0, nodes, subnodes);
        return false;
    }

    return clone_rebalance(parent_id, value, nodes, subnodes);
}

pub fn clone_right<T>(mut node_id: i32, nodes: &mut Nodes<T>, subnodes: &mut Nodes<T>) -> i32
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    if nodes.size == 0 {
        return -1;
    }
    if nodes.size == 1 {
        let root_node = nodes.nodes_map.get_mut(&node_id).unwrap();

        if !subnodes.nodes_map.contains_key(&node_id) {
            subnodes.nodes_map.insert(node_id, root_node.clone());
        }

        if root_node.content.len() == 0 {
            return -1;
        }
    }

    loop {
        let node = nodes.nodes_map.get(&node_id).unwrap();

        if !subnodes.nodes_map.contains_key(&node_id) {
            subnodes.nodes_map.insert(node_id, node.clone());
        }

        if is_leaf(node_id, nodes) {
            return node_id;
        }

        node_id = *node.children_id.last().unwrap();
    }
}

/// leftSibling returns the node's left sibling and child index (in parent) if it exists, otherwise (-1,-1)
/// key is any of keys in node (could even be deleted).
pub fn clone_left_sibling<T>(
    node_id: i32,
    value: &T,
    nodes: &mut Nodes<T>,
    subnodes: &mut Nodes<T>,
) -> (i32, i32)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let node = nodes.nodes_map.get_mut(&node_id).unwrap();
    if !subnodes.nodes_map.contains_key(&node.node_id) {
        subnodes.nodes_map.insert(node.node_id, node.clone());
    }
    let parent_id = node.parent_id;
    if parent_id != -1 {
        let parent_node = nodes.nodes_map.get_mut(&parent_id).unwrap();
        if !subnodes.nodes_map.contains_key(&parent_node.node_id) {
            subnodes
                .nodes_map
                .insert(parent_node.node_id, parent_node.clone());
        }
        let content_slice = parent_node.content.as_slice();
        match content_slice.binary_search(value) {
            Ok(t) => {
                let index = t as i32 - 1;
                if index >= 0 && index < parent_node.children_id.len() as i32 {
                    return (*parent_node.children_id.get(index as usize).unwrap(), index);
                }
            }
            Err(e) => {
                let index = e as i32 - 1;
                if index >= 0 && index < parent_node.children_id.len() as i32 {
                    return (*parent_node.children_id.get(index as usize).unwrap(), index);
                }
            }
        }
    }
    (-1, -1)
}

/// rightSibling returns the node's right sibling and child index (in parent) if it exists, otherwise (-1,-1)
/// key is any of keys in node (could even be deleted).
pub fn clone_right_sibling<T>(
    node_id: i32,
    value: &T,
    nodes: &mut Nodes<T>,
    subnodes: &mut Nodes<T>,
) -> (i32, i32)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let node = nodes.nodes_map.get_mut(&node_id).unwrap();
    if !subnodes.nodes_map.contains_key(&node.node_id) {
        subnodes.nodes_map.insert(node.node_id, node.clone());
    }
    let parent_id = node.parent_id;
    if parent_id != -1 {
        let parent_node = nodes.nodes_map.get_mut(&parent_id).unwrap();
        if !subnodes.nodes_map.contains_key(&parent_node.node_id) {
            subnodes
                .nodes_map
                .insert(parent_node.node_id, parent_node.clone());
        }
        let content_slice = parent_node.content.as_slice();
        match content_slice.binary_search(value) {
            Ok(mut t) => {
                let index = t as i32 + 1;
                if index < parent_node.children_id.len() as i32 {
                    return (*parent_node.children_id.get(index as usize).unwrap(), index);
                }
            }
            Err(e) => {
                let index = e as i32 + 1;
                if index < parent_node.children_id.len() as i32 {
                    return (*parent_node.children_id.get(index as usize).unwrap(), index);
                }
            }
        }
    }
    (-1, -1)
}

pub fn clone_set_parent<T>(
    childrens: &mut Vec<i32>,
    parent_id: i32,
    nodes: &mut Nodes<T>,
    subnodes: &mut Nodes<T>,
) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    for i in childrens {
        let node = nodes.nodes_map.get_mut(&i).unwrap();

        if !subnodes.nodes_map.contains_key(&node.node_id) {
            subnodes.nodes_map.insert(node.node_id, node.clone());
        }

        node.parent_id = parent_id;
    }
    true
}

pub fn verify_subnodes_hash<T>(subnodes: &Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    return verify_nodes_hash(0, subnodes);
}

pub fn verify_nodes_hash<T>(node_id: i32, subnodes: &Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let mut verify_result = false;

    let node = subnodes.nodes_map.get(&node_id).unwrap();
    let mut hash = String::new();
    let mut compute_hash;
    for i in node.content.iter() {
        hash.push_str(i.calculate().as_str());
    }
    let mut looptime = 0;
    for i in node.children_id.iter() {
        if subnodes.nodes_map.contains_key(i) {
            let child_node = subnodes.nodes_map.get(i).unwrap();
            let child_node_hash = verify_nodes_hash(child_node.node_id, subnodes);
            if !child_node_hash {
                panic!("verified failed");
            };
            hash.push_str(child_node.hash.as_str());
        } else {
            hash.push_str(node.children_hash.get(looptime).unwrap())
        }
        looptime = looptime + 1;
    }

    compute_hash = hex::encode(digest::digest(&digest::SHA256, hash.as_ref()));
    if compute_hash == node.hash {
        return true;
    } else {
        return false;
    }
}
