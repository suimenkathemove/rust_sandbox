use crate::node::{create_mock_node, Node};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct FlattenedNodeItem {
    pub id: String,
    pub parent_id: String,
    pub depth: usize,
}

// TODO: Nodeを返したい
pub fn build_node(flattened_node: Vec<FlattenedNodeItem>) -> Rc<RefCell<Node>> {
    let node = Rc::new(RefCell::new(Node {
        id: "root".to_string(),
        children: vec![],
    }));
    let mut map = HashMap::from([(node.borrow().id.to_string(), Rc::clone(&node))]);

    flattened_node.iter().for_each(|item| {
        if !map.contains_key(&item.parent_id) {
            let node = Rc::new(RefCell::new(Node {
                id: item.parent_id.to_string(),
                children: vec![],
            }));
            map.insert(item.parent_id.to_string(), node);
        }

        if !map.contains_key(&item.id) {
            let node = Rc::new(RefCell::new(Node {
                id: item.id.to_string(),
                children: vec![],
            }));
            map.insert(item.id.to_string(), node);
        }

        let parent = map.get(&item.parent_id).unwrap();
        let node = map.get(&item.id).unwrap();
        parent.borrow_mut().children.push(Rc::clone(node));
    });

    node
}

pub fn create_mock_flattened_node() -> Vec<FlattenedNodeItem> {
    vec![
        FlattenedNodeItem {
            id: "1".to_string(),
            parent_id: "root".to_string(),
            depth: 0,
        },
        FlattenedNodeItem {
            id: "4".to_string(),
            parent_id: "1".to_string(),
            depth: 1,
        },
        FlattenedNodeItem {
            id: "10".to_string(),
            parent_id: "4".to_string(),
            depth: 2,
        },
        FlattenedNodeItem {
            id: "11".to_string(),
            parent_id: "4".to_string(),
            depth: 2,
        },
        FlattenedNodeItem {
            id: "12".to_string(),
            parent_id: "4".to_string(),
            depth: 2,
        },
        FlattenedNodeItem {
            id: "5".to_string(),
            parent_id: "1".to_string(),
            depth: 1,
        },
        FlattenedNodeItem {
            id: "6".to_string(),
            parent_id: "1".to_string(),
            depth: 1,
        },
        FlattenedNodeItem {
            id: "2".to_string(),
            parent_id: "root".to_string(),
            depth: 0,
        },
        FlattenedNodeItem {
            id: "7".to_string(),
            parent_id: "2".to_string(),
            depth: 1,
        },
        FlattenedNodeItem {
            id: "8".to_string(),
            parent_id: "2".to_string(),
            depth: 1,
        },
        FlattenedNodeItem {
            id: "9".to_string(),
            parent_id: "2".to_string(),
            depth: 1,
        },
        FlattenedNodeItem {
            id: "3".to_string(),
            parent_id: "root".to_string(),
            depth: 0,
        },
    ]
}

#[test]
fn test_build_node() {
    let node = create_mock_node();
    let flattened_node = create_mock_flattened_node();

    assert_eq!(Rc::new(RefCell::new(node)), build_node(flattened_node));
}
