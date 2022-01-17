use crate::node::{create_mock_node, Node};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct FlattenedNodeItem {
    pub id: String,
    pub parent_id: String,
    pub depth: u32,
}

pub fn build_node(flattened_node: Vec<FlattenedNodeItem>) -> Node {
    let node = Node {
        id: "root".to_string(),
        children: vec![],
    };
    let mut map = HashMap::from([(node.id.to_string(), node.clone())]);

    flattened_node.iter().for_each(|item| {
        let parent_id = item.parent_id.to_string();
        let mut parent = map
            .entry((&parent_id).to_string())
            .or_insert(Node {
                id: (&parent_id).to_string(),
                children: vec![],
            })
            .clone();

        let node = map
            .entry(item.id.to_string())
            .or_insert(Node {
                id: item.id.to_string(),
                children: vec![],
            })
            .clone();

        parent.children.push(node);
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

    assert_eq!(node, build_node(flattened_node));
}
