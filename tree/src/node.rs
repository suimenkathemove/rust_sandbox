use crate::flattened_node::{create_mock_flattened_node, FlattenedNodeItem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub children: Vec<Node>,
}

impl Node {
    pub fn flatten_node(&self) -> Vec<FlattenedNodeItem> {
        let mut flattened_node: Vec<FlattenedNodeItem> = Vec::new();

        fn flatten(
            node: &Node,
            parent_id: &str,
            depth: u32,
            flattened_node: &mut Vec<FlattenedNodeItem>,
        ) {
            flattened_node.push(FlattenedNodeItem {
                id: node.id.to_string(),
                parent_id: parent_id.to_string(),
                depth,
            });

            node.children.iter().for_each(|c| {
                flatten(c, &node.id, depth + 1, flattened_node);
            });
        }
        self.children.iter().for_each(|c| {
            flatten(c, &self.id, 0, &mut flattened_node);
        });

        flattened_node
    }
}

pub fn create_mock_node() -> Node {
    Node {
        id: "root".to_string(),
        children: vec![
            Node {
                id: "1".to_string(),
                children: vec![
                    Node {
                        id: "4".to_string(),
                        children: vec![
                            Node {
                                id: "10".to_string(),
                                children: vec![],
                            },
                            Node {
                                id: "11".to_string(),
                                children: vec![],
                            },
                            Node {
                                id: "12".to_string(),
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        id: "5".to_string(),
                        children: vec![],
                    },
                    Node {
                        id: "6".to_string(),
                        children: vec![],
                    },
                ],
            },
            Node {
                id: "2".to_string(),
                children: vec![
                    Node {
                        id: "7".to_string(),
                        children: vec![],
                    },
                    Node {
                        id: "8".to_string(),
                        children: vec![],
                    },
                    Node {
                        id: "9".to_string(),
                        children: vec![],
                    },
                ],
            },
            Node {
                id: "3".to_string(),
                children: vec![],
            },
        ],
    }
}

#[test]
fn flatten_node() {
    let flattened_node = create_mock_flattened_node();
    let node = create_mock_node();

    assert_eq!(flattened_node, node.flatten_node());
}
