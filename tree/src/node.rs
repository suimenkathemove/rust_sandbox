use crate::flattened_node::{create_mock_flattened_node, FlattenedNodeItem};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct Node {
    pub id: String,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn flatten_node(&self) -> Vec<FlattenedNodeItem> {
        let mut flattened_node = Vec::<FlattenedNodeItem>::new();

        fn flatten(
            flattened_node: &mut Vec<FlattenedNodeItem>,
            node: Rc<RefCell<Node>>,
            parent_id: String,
            depth: usize,
        ) {
            flattened_node.push(FlattenedNodeItem {
                id: node.borrow().id.to_string(),
                parent_id,
                depth,
            });

            node.borrow().children.iter().for_each(|c| {
                flatten(
                    flattened_node,
                    Rc::clone(c),
                    node.borrow().id.to_string(),
                    depth + 1,
                );
            });
        }
        self.children.iter().for_each(|c| {
            flatten(&mut flattened_node, Rc::clone(c), self.id.to_string(), 0);
        });

        flattened_node
    }
}

pub fn create_mock_node() -> Node {
    Node {
        id: "root".to_string(),
        children: vec![
            Rc::new(RefCell::new(Node {
                id: "1".to_string(),
                children: vec![
                    Rc::new(RefCell::new(Node {
                        id: "4".to_string(),
                        children: vec![
                            Rc::new(RefCell::new(Node {
                                id: "10".to_string(),
                                children: vec![],
                            })),
                            Rc::new(RefCell::new(Node {
                                id: "11".to_string(),
                                children: vec![],
                            })),
                            Rc::new(RefCell::new(Node {
                                id: "12".to_string(),
                                children: vec![],
                            })),
                        ],
                    })),
                    Rc::new(RefCell::new(Node {
                        id: "5".to_string(),
                        children: vec![],
                    })),
                    Rc::new(RefCell::new(Node {
                        id: "6".to_string(),
                        children: vec![],
                    })),
                ],
            })),
            Rc::new(RefCell::new(Node {
                id: "2".to_string(),
                children: vec![
                    Rc::new(RefCell::new(Node {
                        id: "7".to_string(),
                        children: vec![],
                    })),
                    Rc::new(RefCell::new(Node {
                        id: "8".to_string(),
                        children: vec![],
                    })),
                    Rc::new(RefCell::new(Node {
                        id: "9".to_string(),
                        children: vec![],
                    })),
                ],
            })),
            Rc::new(RefCell::new(Node {
                id: "3".to_string(),
                children: vec![],
            })),
        ],
    }
}

#[test]
fn flatten_node() {
    let flattened_node = create_mock_flattened_node();
    let node = create_mock_node();

    assert_eq!(flattened_node, node.flatten_node());
}
