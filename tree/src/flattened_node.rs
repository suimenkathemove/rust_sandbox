#[derive(Debug, PartialEq, Eq)]
pub struct FlattenedNodeItem {
    pub id: String,
    pub parent_id: String,
    pub depth: u32,
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